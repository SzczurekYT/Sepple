use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize},
    },
    thread,
};

use sepple::{
    configs::PreconfiguredPipelineParts,
    dictionary::Dictionary,
    model_provider,
    pipeline::{
        Pipeline,
        processor::{
            ipa_processor::IpaProcessor, silero_vad_scorer::SileroVadScorer,
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

    let PreconfiguredPipelineParts {
        audio_chunker,
        sliding_window_config,
        vad_filter,
        sliding_window_chunker,
    } = PreconfiguredPipelineParts::default();

    let window_size_samples = duration_to_sample_count(&sliding_window_config.window_size);
    let (raw_tap, raw_rx) = Tap::<TimestampedVec<f32>>::new(dropped.clone(), Some(vad::CHUNK_SIZE));
    let (post_vad_tap, post_vad_rx) = Tap::new(dropped.clone(), None);
    let (chunk_tap, chunk_rx) =
        Tap::<TimestampedVec<f32>>::new(dropped.clone(), Some(window_size_samples));
    let (token_tap, token_rx) = Tap::new(dropped.clone(), None);

    let reporter = model_provider::indicatif_progress_reporter();
    let model_path = model_provider::ensure_downloaded_and_get_path(&reporter)?;
    let dictionary = Dictionary::from_file(DICTIONARY_PATH)?;

    let ipa_processor = IpaProcessor::init(&model_path, &sliding_window_config)?;

    let (vad_filter, score_rx) = InstrumentedVadFilter::new(vad_filter, dropped.clone());
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
            Pipeline::new(AudioCapture::new(None).expect("audio capture device available"))
        };
        pipeline
            .then(SilenceGate::new(silence_running))
            .then(audio_chunker)
            .then(raw_tap)
            .then(SileroVadScorer::init())
            .then(vad_filter)
            .then(post_vad_tap)
            .then(sliding_window_chunker)
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
