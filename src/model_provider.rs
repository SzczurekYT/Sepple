use std::{
    fs::{self, File},
    io::{self, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    time::Duration,
};

use directories::ProjectDirs;
use reqwest::blocking::Client;
use tempfile::Builder;
use thiserror::Error;

use crate::error::SeppleResult;

#[derive(Debug, Error)]
pub enum ModelSetupError {
    #[error("Failed to create temporary directory for model download.\n{0}")]
    TmpdirCreation(io::Error),
    #[error("Failed to build HTTP client for model download.\n{0}")]
    HttpClientBuild(reqwest::Error),
    #[error("Failed to send model download request.\n{0}")]
    DownloadSend(reqwest::Error),
    #[error("Failed to receive model download response.\n{0}")]
    DownloadReceive(reqwest::Error),
    #[error("Failed to create file for saving model.\n{0}")]
    FileCreate(io::Error),
    #[error("Failed to write model data to file.\n{0}")]
    WriteFile(io::Error),
    #[error("Failed to create directory for saving model.\n{0}")]
    CreateSaveDirectory(io::Error),
    #[error("Failed to move model file to final location.\n{0}")]
    MoveToDestination(io::Error),
}

const MODEL_URL: &str = "https://huggingface.co/SzczurekYT/wav2vec2-large-xlsr-japlmthufielta-ipa1000-ns-bpk/resolve/main/multipa_sim.bpk";
const MODEL_FILE_NAME: &str = "multipa_model.bpk";
const MODEL_SIZE_BYTES: u64 = 1263060956;
pub const MODEL_SIZE: u32 = 10;

pub fn ensure_downloaded_and_get_path() -> SeppleResult<PathBuf> {
    let path = get_model_path();

    if fs::exists(&path).unwrap_or(false) {
        return Ok(path);
    }

    let model_is_ok = fs::metadata(&path)
        .map(|metadata| metadata.size() == MODEL_SIZE_BYTES)
        .unwrap_or(false);

    if model_is_ok {
        return Ok(path);
    }

    download_and_save_model(&path)?;

    Ok(path)
}

pub fn get_model_path() -> PathBuf {
    let base_dir = ProjectDirs::from("yt.szczurek", "", "Sepple")
        .expect("your system is broken, unable to locate home directory ");

    base_dir.cache_dir().to_path_buf().join(MODEL_FILE_NAME)
}

fn download_and_save_model(save_path: &Path) -> Result<(), ModelSetupError> {
    let tmp_dir = Builder::new()
        .prefix("sepple")
        .tempdir()
        .map_err(ModelSetupError::TmpdirCreation)?;

    let path = tmp_dir.path().join(MODEL_FILE_NAME);

    // It it fails it probably doesn't exist
    let _ = fs::remove_file(&path);

    let client = Client::builder()
        .timeout(None)
        .connect_timeout(Duration::from_secs(15))
        .build()
        .map_err(ModelSetupError::HttpClientBuild)?;
    let response = client
        .get(MODEL_URL)
        .send()
        .map_err(ModelSetupError::DownloadSend)?;
    let content = response.bytes().map_err(ModelSetupError::DownloadReceive)?;

    let mut dest = File::create(&path).map_err(ModelSetupError::FileCreate)?;
    dest.write_all(&content)
        .map_err(ModelSetupError::WriteFile)?;

    fs::create_dir_all(save_path.parent().expect("file path to have a parent"))
        .map_err(ModelSetupError::CreateSaveDirectory)?;

    fs::copy(&path, save_path).map_err(ModelSetupError::MoveToDestination)?;
    fs::remove_file(&path).map_err(ModelSetupError::MoveToDestination)?;

    Ok(())
}
