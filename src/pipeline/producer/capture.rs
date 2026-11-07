use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use handy_recorder::{Channels, InputDevice, Recorder, RecorderConfig, Sink, list_input_devices};
use tokio::sync::{Notify, mpsc::Sender};

use crate::{
    error::{SeppleAudioError, SeppleAudioErrorKind},
    pipeline::{PipelineProducer, PipelineSource},
    timestamped_vec::{self, TimestampedVec},
    units::{SAMPLE_RATE_U32, sample_count_to_duration, unix_timestamp_now},
    vad,
};

pub struct AudioCapture {
    controller: Arc<Mutex<AudioCaptureController>>,
}

impl AudioCapture {
    pub fn new(preferred_device: Option<String>) -> Self {
        Self::from_controller(AudioCaptureController::with_device(preferred_device))
    }

    pub fn with_error_handler(
        preferred_device: Option<String>,
        error_handler: impl Fn(SeppleAudioError, CallbackErrorOrigin) + Send + Sync + 'static,
    ) -> Self {
        let controller =
            AudioCaptureController::with_device_and_handler(preferred_device, error_handler);
        Self {
            controller: Arc::new(Mutex::new(controller)),
        }
    }

    fn from_controller(controller: AudioCaptureController) -> Self {
        Self {
            controller: Arc::new(Mutex::new(controller)),
        }
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
            let Ok(mut ctrl) = self.controller.lock() else {
                return;
            };
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

impl From<InputDevice> for CaptureDevice {
    fn from(value: InputDevice) -> Self {
        Self {
            name: value.name,
            id: value.id,
        }
    }
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
];

pub struct AudioCaptureController {
    wanted_device_id: Option<String>,
    recorder: Option<Recorder<HandySink>>,
    sender: Option<Sender<TimestampedVec<f32>>>,
    error_handler:
        Option<Arc<dyn Fn(SeppleAudioError, CallbackErrorOrigin) + Send + Sync + 'static>>,
    pub exit_notification: Arc<Notify>,
}

impl AudioCaptureController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_device(device: Option<String>) -> Self {
        Self {
            wanted_device_id: device,
            ..Default::default()
        }
    }

    pub fn with_device_and_handler(
        device: Option<String>,
        error_handler: impl Fn(SeppleAudioError, CallbackErrorOrigin) + Send + Sync + 'static,
    ) -> Self {
        Self {
            wanted_device_id: device,
            error_handler: Some(Arc::new(error_handler)),
            ..Default::default()
        }
    }

    pub fn capture_device(&self) -> Option<CaptureDevice> {
        self.recorder
            .as_ref()
            .map(|rec| rec.info().clone().device.into())
    }

    pub(crate) fn run(&mut self, sender: Sender<TimestampedVec<f32>>) {
        self.sender = Some(sender.clone());
        let result = self.run_stream(sender);

        let Err(err) = result else {
            return;
        };

        let Some(err_handler) = &self.error_handler else {
            eprintln!("[Sepple] Failed to start mic capture stream!\n{err}");
            return;
        };
        (err_handler)(err, CallbackErrorOrigin::FirstRun)
    }

    pub fn rerun(&mut self) -> Result<(), SeppleAudioError> {
        if let Some(recorder) = self.recorder.take() {
            let _ = recorder.close();
        };

        let Some(sender) = &self.sender else {
            return Ok(());
        };
        self.run_stream(sender.clone())
    }

    fn run_stream(&mut self, sender: Sender<TimestampedVec<f32>>) -> Result<(), SeppleAudioError> {
        let config = RecorderConfig {
            channels: Channels::MixToMono,
            sample_rate: Some(SAMPLE_RATE_U32),
            device: self.wanted_device_id.take(),
            frames_per_chunk: Some(vad::CHUNK_SIZE),
            ..Default::default()
        };

        let err_handler = self.error_handler.as_ref().cloned();

        let recorder = Recorder::open_with_failure_handler(config, move |err| {
            let err = SeppleAudioError::new(err, SeppleAudioErrorKind::MicCaptureError);
            let Some(err_handler) = &err_handler else {
                eprintln!("Audio stream error: {}", err);
                return;
            };
            (err_handler)(err, CallbackErrorOrigin::Stream)
        })
        .map_err(|err| SeppleAudioError::new(err, SeppleAudioErrorKind::MicCaptureSetupError))?;

        let sink = HandySink {
            exit_notification: Arc::clone(&self.exit_notification),
            sender,
        };
        recorder.start(sink).map_err(|err| {
            SeppleAudioError::new(err.error, SeppleAudioErrorKind::MicCaptureStartError)
        })?;

        self.recorder = Some(recorder);

        Ok(())
    }

    /// Sets microphone to the provided id if available
    /// Restarts the stream if needed
    pub fn set_microphone<T>(&mut self, preferred_id: T) -> Result<(), SeppleAudioError>
    where
        T: Into<Option<String>>,
    {
        self.wanted_device_id = preferred_id.into();
        self.rerun()
    }

    pub fn enumerate_microphones() -> Vec<CaptureDevice> {
        let Ok(devices) = list_input_devices() else {
            return vec![];
        };

        let mut device_map: HashMap<String, Vec<InputDevice>> = HashMap::new();
        devices
            .into_iter()
            .filter(Self::device_filter)
            .for_each(|device| {
                device_map
                    .entry(device.name.to_owned())
                    .or_default()
                    .push(device)
            });

        device_map
            .into_values()
            .filter_map(|devices| {
                devices
                    .iter()
                    .find(|dev| dev.is_default || dev.id.to_lowercase().contains("default"))
                    .or_else(|| devices.iter().find(|dev| dev.id.contains("plughw")))
                    .or_else(|| devices.iter().find(|dev| dev.id.contains("front")))
                    .or_else(|| devices.first())
                    .cloned()
            })
            .map(CaptureDevice::from)
            .collect()
    }

    // Filter out devices not really useful to the user
    fn device_filter(device: &InputDevice) -> bool {
        let name = &device.name;

        if device.is_monitor {
            return false;
        }
        if FILTERED_OUT_DEVICE_NAMES.contains(&name.as_str()) {
            return false;
        }
        if FILTERED_OUT_DEVICE_IDS.contains(&device.id.as_str()) {
            return false;
        }

        true
    }
}

impl Default for AudioCaptureController {
    fn default() -> Self {
        Self {
            wanted_device_id: None,
            recorder: None,
            sender: None,
            error_handler: None,
            exit_notification: Arc::new(Notify::new()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CallbackErrorOrigin {
    FirstRun,
    Stream,
}

pub struct HandySink {
    sender: Sender<TimestampedVec<f32>>,
    exit_notification: Arc<Notify>,
}

impl Sink for HandySink {
    fn process_chunk(&mut self, chunk: handy_recorder::AudioChunk<'_>) {
        let data = &chunk.samples[..chunk.valid_frames];
        let timestamp = unix_timestamp_now() - sample_count_to_duration(data.len());

        self.sender
            .blocking_send(timestamped_vec::from_audio_and_timestamp(
                timestamp,
                data.to_vec(),
            ))
            .unwrap_or_else(|_| {
                self.exit_notification.notify_one();
            });
    }
}
