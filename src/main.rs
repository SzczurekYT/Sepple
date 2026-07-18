pub mod debug;
pub mod dictionary;
pub mod ipa_recognizer;
pub mod pipeline;
pub mod timestamped_vec;
pub mod units;
pub mod vad;

use std::{
    collections::HashMap,
    fs,
    iter::{self},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU8, Ordering},
    time::{Duration, Instant},
};

use burn::backend::Flex;
use clap::{Parser, Subcommand};
use hound::{WavReader, WavWriter};

use crate::{
    debug::{assert_string_printer::AssertStringPrinter, audio_logger::AudioLogger},
    dictionary::Dictionary,
    ipa_recognizer::IpaRecognizer,
    pipeline::{
        Pipeline,
        processor::{
            chunker::AudioChunker,
            ipa_processor::IpaProcessor,
            silero_vad_scorer::SileroVadScorer,
            sliding_window::{SlidingWindowChunker, SlidingWindowConfig},
            vad_filter::VadFilter,
            word_detector::WordDetector,
        },
        producer::{capture::AudioCapture, memory_audio_source::MemoryAudioSource},
    },
    units::{SAMPLE_RATE_U32, duration_to_sample_count},
    vad::Vad,
};

pub(crate) static DEBUG: AtomicU8 = AtomicU8::new(0);
pub type SeppleBackend = Flex;

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
        file: PathBuf,
        /// loads audio from file instead of capturing live microphone input
        #[arg(long)]
        silero: bool,
    },
    /// Runs the IPA pipeline
    Pipeline {
        /// loads audio from file instead of capturing live microphone input
        #[arg(short, long, value_name = "FILE")]
        file: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    DEBUG.store(cli.verbose, Ordering::SeqCst);

    match cli.command {
        Command::File { file, silero } => {
            let input = &read_wav_to_f32(file);
            if !silero {
                run_single(input);
            } else {
                run_single_silero(input);
            }
        }
        Command::Pipeline { file } => run_pipeline(file.map(read_wav_to_f32)),
    }
}

fn run_single(input: &[f32]) {
    println!("Loading model");
    let recognizer = IpaRecognizer::<Flex>::init();
    println!("Load done");
    let result = recognizer.recognize(input);
    println!("Result: {result}");
    println!("Words: ");
    let dict = Dictionary::load();
    let words = dict.greedy_search(&result).0;
    for word in words {
        println!("{word}");
    }
}

fn run_single_silero(input: &[f32]) {
    println!("Loading model");
    let mut vad = Vad::<Flex>::init();
    println!("Load done");
    let result = vad.process_audio(input);

    println!("Result: {result:.2?}");
    let audio: Vec<f32> = result
        .iter()
        .flat_map(|prob| iter::repeat_n(*prob, vad::CHUNK_SIZE))
        .collect();
    save_f32_to_wav(&audio, "speech_probabilities.wav");
}

fn run_pipeline(input: Option<Vec<f32>>) {
    let load_start = Instant::now();
    println!("Loading model");
    let sliding_window_config = SlidingWindowConfig {
        window_size: Duration::from_secs(2),
        cut_left: Duration::from_millis(500),
        cut_right: Duration::from_millis(500),
    };
    let vad_scorer = SileroVadScorer::init();
    let ipa_processor = IpaProcessor::init(&sliding_window_config);
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
        .then(AudioChunker::new(vad::CHUNK_SIZE))
        .then(vad_scorer)
        .then(VadFilter::new(0.35, 0.35, 6))
        .then(SlidingWindowChunker::new(
            &sliding_window_config,
            &Duration::from_millis(40),
        ))
        .then(AudioLogger::new(
            Some(duration_to_sample_count(&sliding_window_config.window_size)),
            "debug",
        ))
        .then(ipa_processor)
        .then(word_detector)
        .finish_and_run(AssertStringPrinter::new(vec![
            "prizim".to_owned(),
            "fɛra".to_owned(),
            "kɛjfida".to_owned(),
            "fɛra".to_owned(),
            "kɛjfida".to_owned(),
            "fɛra".to_owned(),
            "kɛjfida".to_owned(),
            "prizim".to_owned(),
            "fɛra".to_owned(),
            "prizim".to_owned(),
            "fɛra".to_owned(),
        ]));
    // .finish_and_run(ValuePrinter::new());
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
