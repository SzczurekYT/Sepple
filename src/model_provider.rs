use std::{fs, path::PathBuf};

use directories::ProjectDirs;

pub fn ensure_downloaded_and_get_path() -> PathBuf {
    let path = get_model_path();

    if fs::exists(&path).unwrap_or(false) {
        return path;
    }

    download_model();

    path
}

pub fn get_model_path() -> PathBuf {
    let base_dir = ProjectDirs::from("yt.szczurek", "", "Sepple")
        .expect("your system is broken, unable to locate home directory ");

    base_dir.cache_dir().to_path_buf().join("multipa_model.bpk")
}

pub fn download_model() {}

fn download_file() {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::blocking::get(target)?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let content = response.bytes()?;
    dest.write_all(&content)?;
    Ok(())
}
