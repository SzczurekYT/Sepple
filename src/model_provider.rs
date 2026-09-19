use std::{
    fs::{self, File},
    io::{self, Read, Write},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    time::Duration,
};

use directories::ProjectDirs;
use indicatif::{ProgressBar, ProgressStyle};
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
    #[error("Failed to read downloaded model data.\n{0}")]
    DownloadRead(io::Error),
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

pub fn ensure_downloaded_and_get_path(
    on_progress: &dyn Fn(u64, Option<u64>),
) -> SeppleResult<PathBuf> {
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

    download_and_save_model(&path, on_progress)?;

    Ok(path)
}

pub fn get_model_path() -> PathBuf {
    let base_dir = ProjectDirs::from("yt.szczurek", "", "Sepple")
        .expect("your system is broken, unable to locate home directory ");

    base_dir.cache_dir().to_path_buf().join(MODEL_FILE_NAME)
}

fn download_and_save_model(
    save_path: &Path,
    on_progress: &dyn Fn(u64, Option<u64>),
) -> Result<(), ModelSetupError> {
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

    let total_size = response.content_length();

    let mut response = response;
    let mut dest = File::create(&path).map_err(ModelSetupError::FileCreate)?;

    let mut downloaded: u64 = 0;
    let mut buf = [0u8; 8192];
    loop {
        let n = response.read(&mut buf).map_err(ModelSetupError::DownloadRead)?;
        if n == 0 {
            break;
        }
        dest.write_all(&buf[..n])
            .map_err(ModelSetupError::WriteFile)?;
        downloaded += n as u64;
        on_progress(downloaded, total_size);
    }

    fs::create_dir_all(save_path.parent().expect("file path to have a parent"))
        .map_err(ModelSetupError::CreateSaveDirectory)?;

    fs::copy(&path, save_path).map_err(ModelSetupError::MoveToDestination)?;
    fs::remove_file(&path).map_err(ModelSetupError::MoveToDestination)?;

    Ok(())
}

pub fn indicatif_progress_reporter() -> impl Fn(u64, Option<u64>) {
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bytes}/{total_bytes} ({bytes_per_sec}) [{elapsed}] [{bar:40}]")
            .unwrap()
            .progress_chars("#>-"),
    );
    move |downloaded, total| {
        if let Some(total) = total {
            pb.set_length(total);
        }
        pb.set_position(downloaded);
    }
}
