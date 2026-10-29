use std::{
    iter::{self},
    path::PathBuf,
    time::Instant,
};

use clap::{Parser, Subcommand};

use sepple::{
    configs::PreconfiguredPipelineParts,
    dictionary::Dictionary,
    error::SeppleResult,
    ipa_recognizer::IpaRecognizer,
    model_provider,
    pipeline::{
        Pipeline,
        consumer::value_printer::ValuePrinter,
        processor::{
            ipa_processor::IpaProcessor, silero_vad_scorer::SileroVadScorer,
            word_detector::WordDetector,
        },
        producer::{capture::AudioCapture, memory_audio_source::MemoryAudioSource},
    },
    read_wav_to_f32, save_f32_to_wav, set_debug,
    vad::{self, Vad},
};

pub const DICTIONARY_PATH: &str = "dictionary.json";

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

    let result = match cli.command {
        Command::File { file, silero } => {
            let input = &read_wav_to_f32(file);
            if !silero {
                run_single(input)
            } else {
                run_single_silero(input);
                Ok(())
            }
        }
        Command::Pipeline { file } => run_pipeline(file.map(read_wav_to_f32)),
    };

    if let Err(err) = result {
        println!("Failed to run sepple, error:\n{err}");
    }
}

fn run_single(input: &[f32]) -> SeppleResult<()> {
    let reporter = model_provider::indicatif_progress_reporter();
    let model_path = model_provider::ensure_downloaded_and_get_path(&reporter)?;
    println!("Loading model");
    let recognizer = IpaRecognizer::init_default(model_path)?;
    println!("Load done");
    let result = recognizer.recognize(input);
    println!("Result: {result}");
    println!("Words: ");
    let dict = Dictionary::from_file(DICTIONARY_PATH).unwrap();
    let words = dict.find_words_in_string(&result).0;
    for word in words {
        println!("{word}");
    }

    Ok(())
}

fn run_single_silero(input: &[f32]) {
    println!("Loading model");
    let mut vad = Vad::init();
    println!("Load done");
    let result = vad.process_audio(input);

    println!("Result: {result:.2?}");
    let audio: Vec<f32> = result
        .iter()
        .flat_map(|prob| iter::repeat_n(*prob, vad::CHUNK_SIZE))
        .collect();
    save_f32_to_wav(&audio, "speech_probabilities.wav");
}

fn run_pipeline(input: Option<Vec<f32>>) -> SeppleResult<()> {
    let load_start = Instant::now();
    let PreconfiguredPipelineParts {
        audio_chunker,
        sliding_window_config,
        vad_filter,
        sliding_window_chunker,
    } = PreconfiguredPipelineParts::default();
    let reporter = model_provider::indicatif_progress_reporter();
    let model_path = model_provider::ensure_downloaded_and_get_path(&reporter)?;
    println!("Loading model");
    let vad_scorer = SileroVadScorer::init();
    let ipa_processor = IpaProcessor::init(model_path, &sliding_window_config)?;
    let word_detector = WordDetector::init(Dictionary::from_file(DICTIONARY_PATH)?);
    println!(
        "Load done (took: {:.2?}), transcribing:",
        load_start.elapsed()
    );

    let pipeline = if let Some(input) = input {
        Pipeline::new(MemoryAudioSource::new(input, vad::CHUNK_SIZE))
    } else {
        Pipeline::new(AudioCapture::new(None)?)
    };

    pipeline
        .then(audio_chunker)
        .then(vad_scorer)
        .then(vad_filter)
        .then(sliding_window_chunker)
        .then(ipa_processor)
        .then(word_detector)
        .build_and_run(ValuePrinter::new());

    Ok(())
}
