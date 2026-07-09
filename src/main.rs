pub mod capture;
pub mod chunker;
pub mod dictionary;
pub mod ipa_processor;
pub mod ipa_recognizer;
pub mod memory_audio_source;
pub mod pipeline;
pub mod sliding_window;
pub mod timestamped_vec;
pub mod units;
pub mod value_printer;
pub mod word_detector;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU8, Ordering},
    time::{Duration, Instant},
};

use burn::backend::Flex;
use clap::{Parser, Subcommand};
use hound::WavReader;

use crate::{
    capture::AudioCapture,
    ipa_processor::IpaProcessor,
    ipa_recognizer::IpaRecognizer,
    memory_audio_source::MemoryAudioSource,
    pipeline::Pipeline,
    sliding_window::{SlidingWindowChunker, SlidingWindowConfig},
    units::SAMPLE_RATE_U32,
    value_printer::ValuePrinter,
    word_detector::WordDetector,
};

pub(crate) static DEBUG: AtomicU8 = AtomicU8::new(0);

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Feeds a file into the ipa detection model and prints result
    File {
        #[arg(short, long, value_name = "FILE")]
        input: PathBuf,
    },
    /// Runs the IPA pipeline
    Pipeline {
        /// loads audio from file instead of capturing live microphone input
        #[arg(short, long, value_name = "FILE")]
        input: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    DEBUG.store(cli.verbose, Ordering::SeqCst);

    match cli.command {
        Command::File { input } => run_single(&read_wav_to_f32(input)),
        Command::Pipeline { input } => run_pipeline(input.map(read_wav_to_f32)),
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
    let load_start = Instant::now();
    println!("Loading model");
    let sliding_window_config = SlidingWindowConfig {
        window_size: Duration::from_secs(2),
        cut_left: Duration::from_millis(500),
        cut_right: Duration::from_millis(500),
    };
    let chunker = SlidingWindowChunker::new(&sliding_window_config);
    let ipa_processor = IpaProcessor::init(sliding_window_config);
    let word_detector = WordDetector::init();
    println!(
        "Load done (took: {:.2?}), transcribing:",
        load_start.elapsed()
    );

    let pipeline = if let Some(input) = input {
        Pipeline::new(MemoryAudioSource::new(input))
    } else {
        Pipeline::new(AudioCapture)
    };

    pipeline
        .then(chunker)
        .then(ipa_processor)
        .then(word_detector)
        .finish_and_run(ValuePrinter::new());
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

pub fn debug_enabled() -> bool {
    DEBUG.load(Ordering::Relaxed) > 0
}
