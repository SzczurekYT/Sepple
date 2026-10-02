#![allow(dead_code)]

use std::{ops::Range, time::Duration};

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub vad_start_threshold: f32,
    pub vad_end_threshold: f32,
    pub window_size: Duration,
    pub cut_left: Duration,
    pub cut_right: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            vad_start_threshold: 0.35,
            vad_end_threshold: 0.35,
            window_size: Duration::from_millis(1000),
            cut_left: Duration::from_millis(150),
            cut_right: Duration::from_millis(150),
        }
    }
}

impl SessionConfig {
    pub fn advance(&self) -> Duration {
        self.window_size - self.cut_left - self.cut_right
    }
}

#[derive(Debug, Clone, Default)]
pub struct TimelineData {
    pub config: SessionConfig,
    origin: Option<Duration>,
    pub raw: AudioSegment,
    pub post_vad: Vec<AudioSegment>,
    pub vad_scores: Vec<VadScoreEvent>,
    pub chunks: Vec<AudioSegment>,
    pub tokens: Vec<TokenEvent>,
    pub snapshots: Vec<BufferSnapshot>,
    pub words: Vec<WordEvent>,
}

impl TimelineData {
    pub fn rebased(&mut self, stamp: Duration) -> Duration {
        let origin = *self.origin.get_or_insert(stamp);
        stamp.saturating_sub(origin)
    }

    pub fn clear(&mut self) {
        self.origin = None;
        self.raw = AudioSegment::default();
        self.post_vad.clear();
        self.vad_scores.clear();
        self.chunks.clear();
        self.tokens.clear();
        self.snapshots.clear();
        self.words.clear();
    }

    pub fn push_raw(&mut self, mut segment: AudioSegment) {
        if self.raw.samples.is_empty() {
            self.raw.start = segment.start;
        }
        segment.start = self.rebased(segment.start);
        self.raw.samples.extend(segment.samples);
    }

    pub fn push_post_vad(&mut self, mut segment: AudioSegment) {
        if segment.samples.is_empty() {
            return;
        }
        segment.start = self.rebased(segment.start);
        self.post_vad.push(segment);
    }

    pub fn push_chunk(&mut self, mut segment: AudioSegment) {
        if segment.samples.is_empty() {
            return;
        }
        segment.start = self.rebased(segment.start);
        self.chunks.push(segment);
    }

    pub fn push_score(&mut self, mut event: VadScoreEvent) {
        let start = self.rebased(event.span.start);
        let len = event.span.end.saturating_sub(event.span.start);
        event.span = start..start + len;
        self.vad_scores.push(event);
    }

    pub fn push_token(&mut self, mut event: TokenEvent) {
        event.time = self.rebased(event.time);
        self.tokens.push(event);
    }

    pub fn push_snapshot(&mut self, mut event: BufferSnapshot) {
        event.at = self.rebased(event.at);
        self.snapshots.push(event);
    }

    pub fn push_word(&mut self, mut event: WordEvent) {
        let start = self.rebased(event.span.start);
        let len = event.span.end.saturating_sub(event.span.start);
        event.span = start..start + len;
        event.detected_at = self.rebased(event.detected_at);
        self.words.push(event);
    }
}

#[derive(Debug, Clone, Default)]
pub struct AudioSegment {
    pub start: Duration,
    pub samples: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct VadScoreEvent {
    pub span: Range<Duration>,
    pub score: f32,
    pub gate_open: bool,
}

#[derive(Debug, Clone)]
pub struct TokenEvent {
    pub time: Duration,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct BufferSnapshot {
    pub at: Duration,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct WordEvent {
    pub word: String,
    pub span: Range<Duration>,
    pub detected_at: Duration,
}
