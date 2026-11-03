use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use directories::ProjectDirs;
#[cfg(feature = "extra")]
use indicatif::{ProgressBar, ProgressStyle};
use tempfile::Builder;
use thiserror::Error;
use ureq::Agent;

use crate::error::SeppleResult;

#[derive(Debug, Error)]
pub enum ModelSetupError {
    #[error("Failed to create temporary directory for model download.\n{0}")]
    TmpdirCreation(io::Error),
    #[error("Failed to send model download request.\n{0}")]
    DownloadRequest(ureq::Error),
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

const MEGABYTE: usize = 1024 * 1024;

pub struct ModelDownloadMeta<'a> {
    url: &'a str,
    file_name: &'a str,
    size_bytes: u64,
}

pub static ZIPA_MODEL_META: ModelDownloadMeta<'_> = ModelDownloadMeta {
    url: "https://huggingface.co/SzczurekYT/zipa-small-crctc-ns-700k-bpk/resolve/main/zipa-small-crctc-ns.bpk",
    file_name: "zipa-small-crctc-ns.bpk",
    size_bytes: 258760700,
};

pub static SILERO_MODEL_META: ModelDownloadMeta<'_> = ModelDownloadMeta {
    url: "https://huggingface.co/SzczurekYT/silero-vad-ifless-bpk/resolve/main/silero_vad_ifless.bpk",
    file_name: "silero_vad_ifless.bpk",
    size_bytes: 2184964,
};

pub fn ensure_downloaded_and_get_path(
    model: &ModelDownloadMeta<'_>,
    on_progress: &dyn Fn(u64, Option<u64>),
) -> SeppleResult<PathBuf> {
    let path = get_model_path(model);

    if fs::exists(&path).unwrap_or(false) {
        return Ok(path);
    }

    let model_is_ok = fs::metadata(&path)
        .map(|metadata| metadata.len() == model.size_bytes)
        .unwrap_or(false);

    if model_is_ok {
        return Ok(path);
    }

    download_and_save_model(model, &path, on_progress)?;

    Ok(path)
}

pub fn get_model_path(model: &ModelDownloadMeta<'_>) -> PathBuf {
    let base_dir = ProjectDirs::from("yt.szczurek", "", "Sepple")
        .expect("your system is broken, unable to locate home directory ");

    base_dir.cache_dir().to_path_buf().join(model.file_name)
}

fn download_and_save_model(
    model: &ModelDownloadMeta<'_>,
    save_path: &Path,
    on_progress: &dyn Fn(u64, Option<u64>),
) -> Result<(), ModelSetupError> {
    let tmp_dir = Builder::new()
        .prefix("sepple")
        .tempdir()
        .map_err(ModelSetupError::TmpdirCreation)?;

    let path = tmp_dir.path().join(model.file_name);

    // It it fails it probably doesn't exist
    let _ = fs::remove_file(&path);

    let agent: Agent = Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(15)))
        .build()
        .into();
    let response = agent
        .get(model.url)
        .call()
        .map_err(ModelSetupError::DownloadRequest)?;

    let (_, body) = response.into_parts();
    let total_size = body.content_length();
    let mut response = body.into_reader();
    let mut dest = File::create(&path).map_err(ModelSetupError::FileCreate)?;

    let mut downloaded: u64 = 0;
    let mut buf = vec![0u8; MEGABYTE];
    loop {
        let n = response
            .read(&mut buf)
            .map_err(ModelSetupError::DownloadRead)?;
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

#[cfg(feature = "extra")]
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
