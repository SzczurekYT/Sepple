use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize},
    },
    thread,
    time::Duration,
};

use sepple::{
    dictionary::Dictionary,
    model_provider,
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
    read_wav_to_f32,
    timestamped_vec::TimestampedVec,
    units::{duration_to_sample_count, unix_timestamp_now},
    vad,
};

use super::instrumentation::{InstrumentedVadFilter, InstrumentedWordDetector, SilenceGate, Tap};
use super::transport::Transport;

const DICTIONARY_PATH: &str = "dictionary.json";

pub fn build(file: Option<PathBuf>) -> Result<Transport, Box<dyn std::error::Error>> {
    let dropped = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(AtomicBool::new(true));
    let silence_running = Arc::clone(&running);

    let window_size = Duration::from_millis(1000);
    let window_size_samples = duration_to_sample_count(&Duration::from_millis(1000));
    let (raw_tap, raw_rx) = Tap::<TimestampedVec<f32>>::new(dropped.clone(), Some(vad::CHUNK_SIZE));
    let (post_vad_tap, post_vad_rx) = Tap::new(dropped.clone(), None);
    let (chunk_tap, chunk_rx) =
        Tap::<TimestampedVec<f32>>::new(dropped.clone(), Some(window_size_samples));
    let (token_tap, token_rx) = Tap::new(dropped.clone(), None);

    let reporter = model_provider::indicatif_progress_reporter();
    let model_path = model_provider::ensure_downloaded_and_get_path(&reporter)?;
    let dictionary = Dictionary::from_file(DICTIONARY_PATH)?;

    let config = SlidingWindowConfig {
        window_size,
        cut_left: Duration::from_millis(150),
        cut_right: Duration::from_millis(150),
    };
    let ipa_processor = IpaProcessor::init(&model_path, &config)?;

    let (vad_filter, score_rx) =
        InstrumentedVadFilter::new(VadFilter::new(0.35, 0.35, 10), dropped.clone());
    let (word_detector, snapshot_rx) =
        InstrumentedWordDetector::new(WordDetector::init(dictionary), dropped.clone());

    let (terminal_tap, word_rx) = Tap::new(dropped.clone(), None);

    thread::spawn(move || {
        let pipeline = if let Some(file) = file {
            Pipeline::new(MemoryAudioSource::new(
                read_wav_to_f32(&file),
                vad::CHUNK_SIZE,
            ))
        } else {
            Pipeline::new(AudioCapture::new().expect("audio capture device available"))
        };
        pipeline
            .then(SilenceGate::new(silence_running))
            .then(AudioChunker::new(vad::CHUNK_SIZE))
            .then(raw_tap)
            .then(SileroVadScorer::init())
            .then(vad_filter)
            .then(post_vad_tap)
            .then(SlidingWindowChunker::new(
                &config,
                &Duration::from_millis(40),
            ))
            .then(chunk_tap)
            .then(ipa_processor)
            .then(token_tap)
            .then(word_detector)
            .build_and_run(terminal_tap);
    });

    Ok(Transport {
        running,
        dropped,
        raw: raw_rx,
        last_raw_update_timestamp: unix_timestamp_now(),
        post_vad: post_vad_rx,
        score: score_rx,
        chunk: chunk_rx,
        token: token_rx,
        snapshot: snapshot_rx,
        word: word_rx,
    })
}
