pub mod capture;
pub mod ipapipeline;
pub mod iparecognizer;

use std::{collections::HashMap, env::args, fs, path::Path, sync::mpsc, time::Duration};

use burn::backend::Flex;
use hound::WavReader;

use crate::{
    ipapipeline::{IpaPipeline, SAMPLE_RATE_U32, SlidingWindowConfig},
    iparecognizer::IpaRecognizer,
};

fn main() {
    let mode = args().nth(1).map(|s| s.to_lowercase());
    let input = args().nth(2).map(read_wav_to_f32);

    match mode.as_deref() {
        Some("single") => {
            run_single(&input.expect("file path for single mode"));
        }
        Some("pipeline") => {
            run_pipeline(input);
        }
        Some(mode) => {
            println!("Invalid mode {mode}");
        }
        None => {
            println!("Please select a mode");
        }
    }
}

fn run_single(input: &[f32]) {
    println!("Loading model");
    let recognizer = IpaRecognizer::<Flex>::init();
    println!("Load done");
    let result = recognizer.recognize(input);
    println!("Result: {result}");
}

fn run_pipeline(input: Option<Vec<f32>>) {
    let audio_rx = if let Some(input) = input {
        let (tx, rx) = mpsc::channel::<Vec<f32>>();
        tx.send(input).unwrap();
        rx
    } else {
        capture::start_audio_capture(Duration::from_secs(1))
    };
    println!("Loading model");
    let sliding_window_config = SlidingWindowConfig {
        window_size: Duration::from_secs(2),
        stride: Duration::from_millis(500),
    };
    let mut pipeline = IpaPipeline::init(sliding_window_config);
    println!("Load done");
    pipeline.run(audio_rx);
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
