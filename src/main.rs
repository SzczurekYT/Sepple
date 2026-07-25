use std::{
    iter::{self},
    path::PathBuf,
    time::{Duration, Instant},
};

use burn::backend::Flex;
use clap::{Parser, Subcommand};

use sepple::{
    debug::{assert_string_printer::AssertStringPrinter, audio_logger::AudioLogger},
    dictionary::{DEFAULT_CONFUSION_DISTANCE_THRESHOLD, Dictionary},
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
    read_wav_to_f32, save_f32_to_wav, set_debug,
    units::duration_to_sample_count,
    vad::{self, Vad},
};

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

    set_debug(cli.verbose);

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
    let dict = Dictionary::load(DEFAULT_CONFUSION_DISTANCE_THRESHOLD);
    let words = dict.find_words_in_string(&result).0;
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
    let word_detector = WordDetector::init(DEFAULT_CONFUSION_DISTANCE_THRESHOLD);
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
        .then(VadFilter::new(0.35, 0.35, 10))
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
