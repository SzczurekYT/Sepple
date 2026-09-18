use std::io;

use burn_store::BurnpackError;
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
    #[error("The system has no input audio devices. You need a microphone.")]
    NoMicFound,
    #[error("Cannot load the speech recognition model, burnpack error occured\n{0}")]
    MultipaModelLoad(#[from] BurnpackError),
    #[error("Model setup failed\n{0}")]
    ModelSetup(#[from] ModelSetupError),
}
