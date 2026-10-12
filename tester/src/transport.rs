use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};

use sepple::{
    pipeline::processor::{
        ipa_processor::TimestampedText, vad_filter::VadValue, word_detector::DetectedWord,
    },
    timestamped_vec::TimestampedVec,
};
use tokio::sync::mpsc::Receiver;
use unicode_segmentation::UnicodeSegmentation;

use crate::gui::timeline_data::{
    AudioSegment, BufferSnapshot, TimelineData, TokenEvent, VadScoreEvent, WordEvent,
};

pub struct Transport {
    pub running: Arc<AtomicBool>,
    pub dropped: Arc<AtomicUsize>,
    pub raw: Receiver<TimestampedVec<f32>>,
    pub post_vad: Receiver<VadValue>,
    pub score: Receiver<VadScoreEvent>,
    pub chunk: Receiver<TimestampedVec<f32>>,
    pub token: Receiver<TimestampedText>,
    pub snapshot: Receiver<BufferSnapshot>,
    pub word: Receiver<DetectedWord>,
}

impl Transport {
    pub fn dropped(&self) -> usize {
        self.dropped.load(Ordering::Relaxed)
    }

    pub fn drain(&mut self, timeline: &mut TimelineData) {
        drain_feed(
            &mut self.raw,
            timeline,
            |event| event.first().map(|(_, time)| *time),
            |timeline, value| timeline.push_raw(value.into()),
        );
        drain_feed(
            &mut self.post_vad,
            timeline,
            vad_stamp,
            |timeline, value| {
                if let Some(segment) = vad_audio(value) {
                    timeline.push_post_vad(segment);
                }
            },
        );
        drain_feed(
            &mut self.score,
            timeline,
            |event| Some(event.span.start),
            |timeline, value| timeline.push_score(value),
        );
        drain_feed(
            &mut self.chunk,
            timeline,
            |event| event.first().map(|(_, t)| *t),
            |timeline, value| timeline.push_chunk(value.into()),
        );
        drain_feed(
            &mut self.token,
            timeline,
            |event| Some(event.start_time()),
            |timeline, value| {
                for event in token_events(value) {
                    timeline.push_token(event);
                }
            },
        );
        drain_feed(
            &mut self.snapshot,
            timeline,
            |event| Some(event.at),
            |timeline, value| timeline.push_snapshot(value),
        );
        drain_feed(
            &mut self.word,
            timeline,
            |event| Some(event.audio_span.start),
            |timeline, value| timeline.push_word(value.into()),
        );
    }
}

fn drain_feed<T>(
    receiver: &mut Receiver<T>,
    timeline: &mut TimelineData,
    stamp_of: impl Fn(&T) -> Option<Duration>,
    push: impl Fn(&mut TimelineData, T),
) {
    while let Ok(event) = receiver.try_recv() {
        if let Some(origin) = timeline.origin
            && let Some(stamp) = stamp_of(&event)
            && stamp < origin
        {
            continue;
        }
        push(timeline, event);
    }
}

impl From<TimestampedVec<f32>> for AudioSegment {
    fn from(audio: TimestampedVec<f32>) -> Self {
        let start = audio.first().map(|(_, t)| *t).unwrap_or_default();
        AudioSegment {
            start,
            samples: audio.into_iter().map(|(sample, _)| sample).collect(),
        }
    }
}

impl From<DetectedWord> for WordEvent {
    fn from(word: DetectedWord) -> Self {
        WordEvent {
            word: word.word,
            span: word.audio_span,
            detected_at: word.detected_at,
        }
    }
}

fn vad_stamp(value: &VadValue) -> Option<Duration> {
    match value {
        VadValue::Data(audio) => audio.first().map(|(_, t)| *t),
        VadValue::SpeechEnd => None,
    }
}

fn vad_audio(value: VadValue) -> Option<AudioSegment> {
    match value {
        VadValue::Data(audio) => Some(audio.into()),
        VadValue::SpeechEnd => None,
    }
}

fn token_events(text: TimestampedText) -> Vec<TokenEvent> {
    let start = text.start_time();
    let end = text.end_time();
    let graphemes: Vec<&str> = text.text.graphemes(true).collect();
    let count = graphemes.len();
    if count == 0 {
        return Vec::new();
    }
    let duration = end.saturating_sub(start);
    graphemes
        .iter()
        .enumerate()
        .map(|(i, symbol)| {
            let time = start + duration * i as u32 / count as u32;
            TokenEvent {
                time,
                symbol: symbol.to_string(),
            }
        })
        .collect()
}
