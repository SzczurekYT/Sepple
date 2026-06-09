pub mod capture;
pub mod ipapipeline;
pub mod iparecognizer;

use std::{collections::HashMap, fs, path::Path, time::Instant};

use hound::WavReader;

use crate::ipapipeline::{IpaPipeline, SAMPLE_RATE_U32};

fn main() {
    let audio_tx = capture::start_audio_capture();
    let mut pipeline = IpaPipeline::init(audio_tx);
    pipeline.run();
}

pub fn load_vocab(path: &str) -> HashMap<usize, String> {
    let data = fs::read_to_string(path).expect("Unable to read vocab.json");
    let map: HashMap<String, usize> =
        serde_json::from_str(&data).expect("Invalid vocab.json format");

    map.into_iter().map(|(token, id)| (id, token)).collect()
}

pub fn read_wav_to_f32<P: AsRef<Path>>(path: P) -> Vec<f32> {
    let mut reader = WavReader::open(path).expect("Failed to open WAV file");
    let spec = reader.spec();
    assert_eq!(spec.channels, 1, "Audio must be mono");
    assert_eq!(
        spec.sample_rate, SAMPLE_RATE_U32,
        "Sample rate must be 16 kHz"
    );
    assert_eq!(spec.bits_per_sample, 16, "Only 16‑bit PCM is supported");

    // normalize to [-1.0, 1.0]
    reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / (i16::MAX as f32 + 1.0))
        .collect()
}

fn wrap_elapsed<R>(action: &str, f: impl FnOnce() -> R) -> R {
    let now = Instant::now();

    let result = f();

    let elapsed = now.elapsed();
    println!("{action} took {:.2?}", elapsed);
    result
}
