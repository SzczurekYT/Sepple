pub mod capture;
pub mod ipapipeline;
pub mod iparecognizer;

use std::{collections::HashMap, fs, path::Path, time::Duration};

use hound::WavReader;

use crate::ipapipeline::{IpaPipeline, SAMPLE_RATE_U32, SlidingWindowConfig};

fn main() {
    let sliding_window_config = SlidingWindowConfig {
        window_size: Duration::from_secs(2),
        stride: Duration::from_millis(500),
    };
    let mut pipeline = IpaPipeline::init(sliding_window_config);
    let audio_tx = capture::start_audio_capture(Duration::from_secs(1));
    pipeline.run(audio_tx);
}

// fn main() {
//     let recognizer = IpaRecognizer::<Flex>::init();
//     let samples = read_wav_to_f32("test.wav");
//     let normalized = z_score_normalize(&samples);
//     let result = recognizer.process(&normalized);
//     let result = recognizer.greedy_ctc_decode(&result);
//     let result = recognizer.decode_tokens(&result);
//     println!("Result: {result}");
// }

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
