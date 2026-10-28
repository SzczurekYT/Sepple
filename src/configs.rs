use crate::{
    pipeline::processor::{
        chunker::AudioChunker,
        sliding_window::{SlidingWindowChunker, SlidingWindowConfig},
        vad_filter::VadFilter,
    },
    units::logit_count_to_time,
    vad,
};

pub struct PreconfiguredPipelineParts {
    pub audio_chunker: AudioChunker,
    pub sliding_window_config: SlidingWindowConfig,
    pub vad_filter: VadFilter,
    pub sliding_window_chunker: SlidingWindowChunker,
}

impl Default for PreconfiguredPipelineParts {
    fn default() -> Self {
        let sliding_window_config = SlidingWindowConfig::default();
        let sliding_window_chunker =
            SlidingWindowChunker::new(&sliding_window_config, &logit_count_to_time(2));
        Self {
            sliding_window_config: SlidingWindowConfig::default(),
            audio_chunker: AudioChunker::new(vad::CHUNK_SIZE),
            vad_filter: VadFilter::default(),
            sliding_window_chunker,
        }
    }
}
