pub mod debug;
pub mod dictionary;
pub mod ipa_recognizer;
pub mod pipeline;
pub mod timestamped_vec;
pub mod units;
pub mod vad;

use std::{
    path::Path,
    sync::atomic::{AtomicU8, Ordering},
};

use burn::backend::Flex;
use hound::{WavReader, WavWriter};

use crate::units::SAMPLE_RATE_U32;

pub(crate) static DEBUG: AtomicU8 = AtomicU8::new(0);
pub type SeppleBackend = Flex;

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

pub fn save_f32_to_wav<P: AsRef<Path>>(samples: &[f32], path: P) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE_U32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(path, spec).expect("Failed to open WAV file");
    for sample in samples {
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

pub fn debug_enabled() -> bool {
    DEBUG.load(Ordering::Relaxed) > 0
}

pub fn set_debug(debug_level: u8) {
    DEBUG.store(debug_level, Ordering::SeqCst);
}
