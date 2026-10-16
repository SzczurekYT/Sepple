//! Compares the Rust Zipa pipeline against dumps from `python/zipa_reference.py`.
//!
//! ```shell
//! cargo run --release --bin zipa_compare -- test.wav /tmp/opencode/ref
//! ```

use std::{env::args, fs, path::PathBuf};

use burn::prelude::*;
use sepple::{fbank::Fbank, ipa_recognizer::IpaRecognizer, read_wav_to_f32};

const MODEL_PATH: &str = "model/zipa-small-crctc-ns.bpk";

fn read_f32(path: &PathBuf) -> Vec<f32> {
    fs::read(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
        .as_chunks::<4>()
        .0
        .iter()
        .map(|bytes| f32::from_le_bytes(*bytes))
        .collect()
}

fn read_i32(path: &PathBuf) -> Vec<i32> {
    fs::read(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
        .as_chunks::<4>()
        .0
        .iter()
        .map(|bytes| i32::from_le_bytes(*bytes))
        .collect()
}

fn max_abs_diff(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "length mismatch: {} vs {}", a.len(), b.len());
    a.iter()
        .zip(b)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max)
}

fn report_f32(name: &str, rust: &[f32], reference: &[f32]) {
    let diff = max_abs_diff(rust, reference);
    println!("{name:>10}: len={:>8} max_abs_diff={diff:.3e}", rust.len());
}

fn report_i32(name: &str, rust: &[i32], reference: &[i32]) {
    let mismatches = rust
        .iter()
        .zip(reference)
        .filter(|(a, b)| a != b)
        .count()
        + rust.len().abs_diff(reference.len());
    println!(
        "{name:>10}: len={:>8} (ref {:>8}) mismatches={mismatches}",
        rust.len(),
        reference.len()
    );
}

fn main() {
    let wav = args().nth(1).expect("usage: zipa_compare <wav> <ref_dir>");
    let ref_dir = PathBuf::from(args().nth(2).expect("usage: zipa_compare <wav> <ref_dir>"));

    let samples = read_wav_to_f32(&wav);
    println!("{wav}: {} samples", samples.len());

    let fbank = Fbank::new(Device::default());
    let features = fbank.compute_vec(&samples);
    let reference_features = read_f32(&ref_dir.join("fbank.f32"));
    report_f32("fbank", &features, &reference_features);

    let recognizer = IpaRecognizer::init_default(MODEL_PATH).expect("model to load");

    let (log_probs, valid) = recognizer.forward(&samples);
    let dimensions = log_probs.dims();
    let log_probs: Vec<f32> = log_probs
        .slice(s![0..valid, ..])
        .into_data()
        .try_into_vec_as::<f32>()
        .expect("log probs to be f32");
    println!("log_probs shape: [{valid}, {}]", dimensions[1]);
    let reference_log_probs = read_f32(&ref_dir.join("log_probs.f32"));
    report_f32("log_probs", &log_probs, &reference_log_probs);

    let ids = recognizer.process(&samples);
    let reference_ids = read_i32(&ref_dir.join("ids.i32"));
    report_i32("ids", &ids, &reference_ids);

    let text = recognizer.recognize(&samples);
    let reference_text = fs::read_to_string(ref_dir.join("text.txt")).expect("reference text");
    println!("      text: {}", if text == reference_text { "MATCH" } else { "DIFFERS" });
    if text != reference_text {
        println!("   rust: {text}");
        println!("    ref: {reference_text}");
    }
}
