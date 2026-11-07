use std::{
    fmt::{self, Display},
    io,
};

use burn_store::burn_pack::Error as BurnpackError;
use handy_recorder::{Error as RecorderError, ErrorKind};
use thiserror::Error;

use crate::model_provider::ModelSetupError;

pub type SeppleResult<T> = Result<T, SeppleError>;

#[derive(Debug, Error)]
pub enum SeppleError {
    #[error("Cannot load {thing} from {path}, os error occured\n{error}")]
    FileLoad {
        thing: &'static str,
        path: String,
        error: io::Error,
    },
    #[error("Cannot deserialize {thing} from {path}, json error occured\n{error}")]
    FileDeserialization {
        thing: &'static str,
        path: String,
        error: serde_json::Error,
    },
    #[error(transparent)]
    Audio(#[from] SeppleAudioError),
    #[error("Cannot load the speech recognition model, burnpack error occured\n{0}")]
    ModelLoad(#[from] BurnpackError),
    #[error("Model setup failed\n{0}")]
    ModelSetup(#[from] ModelSetupError),
}

#[derive(Debug, Error)]
pub struct SeppleAudioError {
    error: RecorderError,
    #[source]
    kind: SeppleAudioErrorKind,
}

impl SeppleAudioError {
    pub fn new(error: RecorderError, kind: SeppleAudioErrorKind) -> Self {
        Self { error, kind }
    }
}

impl Display for SeppleAudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.kind)?;
        let error = &self.error;
        if f.alternate() {
            write!(f, "{}", self.error)
        } else {
            let binding = error.device();
            let device = binding.as_ref().filter(|_| is_about_device(error.kind()));
            if let Some(device) = device {
                write!(f, "{} ", device.name)?;
            }
            f.write_str(describe(error.kind(), device.is_some()))
        }
    }
}

#[derive(Debug, Error)]
pub enum SeppleAudioErrorKind {
    #[error("Failed to setup microphone capture")]
    MicCaptureSetupError,
    #[error("Failed to start microphone capture")]
    MicCaptureStartError,
    #[error("Error occured during microphone capture")]
    MicCaptureError,
}

// COPY PASTED CODE
// from `handy-recorder`
// because it is private

fn is_about_device(kind: ErrorKind) -> bool {
    !matches!(
        kind,
        ErrorKind::SinkStalled
            | ErrorKind::Processing
            | ErrorKind::AlreadyRecording
            | ErrorKind::NotRecording
            | ErrorKind::StopFromSink
    )
}

fn describe(kind: ErrorKind, after_device: bool) -> &'static str {
    match (kind, after_device) {
        (ErrorKind::DeviceUnavailable, true) => "is not available",
        (ErrorKind::DeviceUnavailable, false) => "no input device is available",
        (ErrorKind::DeviceBusy, true) => "is in use by another application",
        (ErrorKind::DeviceBusy, false) => "the input device is in use by another application",
        (ErrorKind::PermissionDenied, true) => "access was denied",
        (ErrorKind::PermissionDenied, false) => "microphone access was denied",
        (ErrorKind::UnsupportedFormat, true) => "cannot deliver the requested format",
        (ErrorKind::UnsupportedFormat, false) => "the requested format is not supported",
        (ErrorKind::InvalidChannel, true) => "has no such channel",
        (ErrorKind::InvalidChannel, false) => "the requested channel does not exist",
        (ErrorKind::OpenTimedOut, true) => "did not finish opening in time",
        (ErrorKind::OpenTimedOut, false) => "the input device did not finish opening in time",
        (ErrorKind::NoAudio, true) => "never delivered audio",
        (ErrorKind::NoAudio, false) => "the input device never delivered audio",
        (ErrorKind::DeviceLost, true) => "disconnected",
        (ErrorKind::DeviceLost, false) => "the input device disconnected",
        (ErrorKind::StreamInvalidated, true) => "stream was invalidated",
        (ErrorKind::StreamInvalidated, false) => "the input stream was invalidated",
        (ErrorKind::Stalled, true) => "stopped delivering audio",
        (ErrorKind::Stalled, false) => "the input device stopped delivering audio",
        (ErrorKind::SinkStalled, _) => "the sink stopped returning",
        (ErrorKind::Backend, true) => "reported an error",
        (ErrorKind::Backend, false) => "the audio backend reported an error",
        (ErrorKind::Processing, _) => "audio processing failed (a bug in handy-recorder)",
        (ErrorKind::AlreadyRecording, _) => "a recording is already active",
        (ErrorKind::NotRecording, _) => "there is no recording to stop",
        (ErrorKind::StopFromSink, _) => "stop cannot be called from inside the sink",
        (ErrorKind::CloseTimedOut, true) => "did not finish closing in time",
        (ErrorKind::CloseTimedOut, false) => "the input device did not finish closing in time",
        _ => "unknown error",
    }
}
