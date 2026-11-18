use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
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
    controller: AudioCaptureController,
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
        Self { controller }
    }

    fn from_controller(controller: AudioCaptureController) -> Self {
        Self { controller }
    }

    pub fn controller(&self) -> AudioCaptureController {
        self.controller.clone()
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
        self.controller.run(sender);
        let exit_notification = Arc::clone(&self.controller.exit_notification);
        exit_notification.notified().await
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

#[derive(Default, Clone)]
pub struct AudioCaptureController {
    state: Arc<Mutex<AudioCaptureState>>,
    error_handler:
        Option<Arc<dyn Fn(SeppleAudioError, CallbackErrorOrigin) + Send + Sync + 'static>>,
    pub exit_notification: Arc<Notify>,
}

#[derive(Default)]
struct AudioCaptureState {
    wanted_device_id: Option<String>,
    recorder: Option<Recorder<HandySink>>,
    sender: Option<Sender<TimestampedVec<f32>>>,
}

impl AudioCaptureController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_device(device: Option<String>) -> Self {
        Self {
            state: Arc::new(Mutex::new(AudioCaptureState {
                wanted_device_id: device,
                ..Default::default()
            })),
            ..Default::default()
        }
    }

    pub fn with_device_and_handler(
        device: Option<String>,
        error_handler: impl Fn(SeppleAudioError, CallbackErrorOrigin) + Send + Sync + 'static,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(AudioCaptureState {
                wanted_device_id: device,
                ..Default::default()
            })),
            error_handler: Some(Arc::new(error_handler)),
            ..Default::default()
        }
    }

    fn state(&self) -> MutexGuard<'_, AudioCaptureState> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn capture_device(&self) -> Option<CaptureDevice> {
        self.state()
            .recorder
            .as_ref()
            .map(|rec| rec.info().clone().device.into())
    }

    pub(crate) fn run(&self, sender: Sender<TimestampedVec<f32>>) {
        self.state().sender = Some(sender.clone());
        let result = self.run_stream(sender);

        let Err(err) = result else {
            return;
        };

        let Some(err_handler) = &self.error_handler else {
            eprintln!("[Sepple] Failed to start mic capture stream!\n{err}");
            return;
        };
        (err_handler)(err, CallbackErrorOrigin::FirstRun);
    }

    pub fn rerun(&self) -> Result<(), SeppleAudioError> {
        let sender = {
            let mut state = self.state();
            if let Some(recorder) = state.recorder.take() {
                let _ = recorder.close();
            }
            state.sender.clone()
        };

        let Some(sender) = sender else {
            return Ok(());
        };
        self.run_stream(sender)
    }

    fn run_stream(&self, sender: Sender<TimestampedVec<f32>>) -> Result<(), SeppleAudioError> {
        let mut state = self.state();
        let config = RecorderConfig {
            channels: Channels::MixToMono,
            sample_rate: Some(SAMPLE_RATE_U32),
            device: state.wanted_device_id.clone(),
            frames_per_chunk: Some(vad::CHUNK_SIZE),
            ..Default::default()
        };

        let err_handler = self.error_handler.clone();

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

        state.recorder = Some(recorder);

        Ok(())
    }

    /// Sets microphone to the provided id if available
    /// Restarts the stream if needed
    pub fn set_microphone<T>(&self, preferred_id: T) -> Result<(), SeppleAudioError>
    where
        T: Into<Option<String>>,
    {
        self.state().wanted_device_id = preferred_id.into();
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
