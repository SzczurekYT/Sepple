use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use cpal::{
    Device, DeviceDescription, DeviceDescriptionBuilder, DeviceId, ErrorKind, Host, Stream,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use tokio::sync::{Notify, mpsc::Sender};

use crate::{
    error::{SeppleError, SeppleResult},
    pipeline::{PipelineProducer, PipelineSource},
    timestamped_vec::{self, TimestampedVec},
    units::{SAMPLE_RATE_U32, sample_count_to_duration, unix_timestamp_now},
};

pub struct AudioCapture {
    controller: Arc<Mutex<AudioCaptureController>>,
}

impl AudioCapture {
    pub fn new(preferred_device: Option<String>) -> SeppleResult<Self> {
        let mut controller = AudioCaptureController::new();
        controller.set_microphone(preferred_device)?;

        Ok(Self {
            controller: Arc::new(Mutex::new(controller)),
        })
    }

    pub fn controller(&self) -> Arc<Mutex<AudioCaptureController>> {
        Arc::clone(&self.controller)
    }
}

impl PipelineProducer for AudioCapture {
    type Output = TimestampedVec<f32>;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineSource for AudioCapture {
    fn name() -> &'static str {
        "AudioCapture"
    }

    async fn run(&mut self, sender: Sender<Self::Output>) {
        // Written like this so guard is dropped before await
        let exit_notification = {
            let mut ctrl = self.controller.lock().unwrap();
            ctrl.run(sender);
            Arc::clone(&ctrl.exit_notification)
        };

        exit_notification.notified().await;
    }
}

#[derive(Debug, Clone)]
pub struct CaptureDevice {
    pub name: String,
    pub id: String,
}

const FILTERED_OUT_DEVICE_NAMES: &[&str] = &[
    // Pipewire
    "default_input",
    "unknown",
];
const FILTERED_OUT_DEVICE_IDS: &[&str] = &[
    // ALSA
    "alsa:null",
    "alsa:pipewire",
    "alsa:pulse",
    "alsa:default",
];

pub struct AudioCaptureController {
    host: Host,
    device: Option<Device>,
    stream: Option<Stream>,
    sender: Option<Sender<TimestampedVec<f32>>>,
    pub exit_notification: Arc<Notify>,
}

impl AudioCaptureController {
    pub fn new() -> Self {
        Self {
            host: cpal::default_host(),
            device: None,
            stream: None,
            sender: None,
            exit_notification: Arc::new(Notify::new()),
        }
    }

    pub(crate) fn run(&mut self, sender: Sender<TimestampedVec<f32>>) {
        self.sender = Some(sender.clone());
        let result = self.run_stream(sender);
        if let Err(err) = result {
            eprintln!("[Sepple] Failed to start mic capture stream!\n{err}");
        }
    }

    pub fn rerun(&mut self) -> SeppleResult<()> {
        if let Some(stream) = self.stream.take() {
            let _ = stream.pause();
            // Let's be sure, this might release something before
            // we try using that thing again
            drop(stream);
        };

        let Some(sender) = &self.sender else {
            return Ok(());
        };
        self.run_stream(sender.clone())
    }

    fn run_stream(&mut self, sender: Sender<TimestampedVec<f32>>) -> SeppleResult<()> {
        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: SAMPLE_RATE_U32,
            buffer_size: cpal::BufferSize::Default,
        };

        let Some(ref device) = self.device else {
            return Ok(());
        };

        let exit_notification2 = Arc::clone(&self.exit_notification);
        let exit_notification3 = Arc::clone(&self.exit_notification);

        let stream = device
            .build_input_stream(
                config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let timestamp = unix_timestamp_now() - sample_count_to_duration(data.len());

                    // Normalize i16 to f32 [-1.0, 1.0]
                    let normalized: Vec<f32> = data
                        .iter()
                        .map(|&s| s as f32 / (i16::MAX as f32 + 1.0))
                        .collect();

                    sender
                        .blocking_send(timestamped_vec::from_audio_and_timestamp(
                            timestamp, normalized,
                        ))
                        .unwrap_or_else(|_| {
                            exit_notification2.notify_one();
                        });
                },
                move |err| {
                    if err.kind() == ErrorKind::DeviceChanged {
                        return;
                    }
                    eprintln!("Audio stream error: {}", err);
                    exit_notification3.notify_one();
                },
                None,
            )
            .map_err(SeppleError::MicCaptureSetupError)?;

        stream.play().map_err(SeppleError::MicCaptureStartError)?;
        self.stream = Some(stream);

        Ok(())
    }

    /// Sets microphone to the provided id if available, or use a default device
    /// Restarts the stream if needed
    pub fn set_microphone<T>(&mut self, preferred_id: T) -> SeppleResult<()>
    where
        T: Into<Option<String>>,
    {
        let current_device = self.device.take();
        let device = self
            .get_device_by_id(preferred_id.into())
            .or_else(|| self.host.default_input_device());
        if device.is_some() {
            self.device = device;
            let rerun_result = self.rerun();
            if rerun_result.is_ok() {
                return Ok(());
            }
            if current_device.is_none() {
                return rerun_result;
            };
            self.device = current_device;
            self.rerun()
        } else {
            Err(SeppleError::NoMicFound)
        }
    }

    fn get_device_by_id(&self, id: Option<String>) -> Option<Device> {
        let host = cpal::default_host();
        let id: DeviceId = id?.parse().ok()?;
        host.device_by_id(&id)
    }

    pub fn enumerate_microphones() -> Vec<CaptureDevice> {
        let host = cpal::default_host();
        let Ok(devices) = host.input_devices() else {
            return vec![];
        };

        let mut device_map: HashMap<String, Vec<(DeviceDescription, String)>> = HashMap::new();
        devices
            .filter_map(|d| {
                let id = d.id().ok()?.to_string();
                let desc = d.description().ok()?;
                Some((desc, id))
            })
            .filter_map(Self::device_filter)
            .for_each(|device| {
                device_map
                    .entry(device.0.name().to_owned())
                    .or_default()
                    .push(device)
            });

        device_map
            .into_values()
            .map(|devices| {
                let default = devices.iter().find(|(desc, _)| {
                    desc.extended()
                        .any(|line| line.to_lowercase().contains("default"))
                });
                if let Some(default) = default {
                    return default.clone();
                }
                let front: Option<&(DeviceDescription, String)> =
                    devices.iter().find(|(desc, _)| {
                        desc.extended()
                            .any(|line| line.to_lowercase().contains("front"))
                    });
                if let Some(front) = front {
                    return front.clone();
                }
                let plug: Option<&(DeviceDescription, String)> =
                    devices.iter().find(|(desc, _)| {
                        desc.driver()
                            .is_some_and(|driver| driver.contains("plughw"))
                    });
                if let Some(plug) = plug {
                    return plug.clone();
                }
                devices.first().expect("at least one variant").clone()
            })
            .map(|(desc, id)| CaptureDevice {
                name: desc.name().to_owned(),
                id,
            })
            .collect()
    }

    // Filter out devices not really useful to the user
    fn device_filter(
        (desc, id): (DeviceDescription, String),
    ) -> Option<(DeviceDescription, String)> {
        let mut name = desc.name().to_owned();

        if FILTERED_OUT_DEVICE_NAMES.contains(&name.as_str()) {
            return None;
        }
        if FILTERED_OUT_DEVICE_IDS.contains(&id.as_str()) {
            return None;
        }

        // Pipewire
        if name == "default_sink" {
            name = "Default".to_owned();
        }
        if desc
            .driver()
            .is_some_and(|driver| driver == "api.alsa.pcm.sink")
        {
            return None;
        }

        // The only way to set name :(
        let mut desc_builder = DeviceDescriptionBuilder::new(name);
        if let Some(mf) = desc.manufacturer() {
            desc_builder = desc_builder.manufacturer(mf);
        }
        if let Some(driver) = desc.driver() {
            desc_builder = desc_builder.driver(driver);
        }
        desc_builder = desc_builder.device_type(desc.device_type());
        desc_builder = desc_builder.interface_type(desc.interface_type());
        desc_builder = desc_builder.direction(desc.direction());
        if let Some(address) = desc.address() {
            desc_builder = desc_builder.address(address);
        }
        desc_builder = desc_builder.extended(desc.extended());

        Some((desc_builder.build(), id))
    }
}

impl Default for AudioCaptureController {
    fn default() -> Self {
        Self::new()
    }
}
