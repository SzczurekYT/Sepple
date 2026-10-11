use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use tokio::sync::mpsc::Receiver;

use super::timeline_data::{
    AudioSegment, BufferSnapshot, TimelineData, TokenEvent, VadScoreEvent, WordEvent,
};

pub struct Transport {
    pub dropped: Arc<AtomicUsize>,
    pub raw: Receiver<AudioSegment>,
    pub post_vad: Receiver<AudioSegment>,
    pub score: Receiver<VadScoreEvent>,
    pub chunk: Receiver<AudioSegment>,
    pub token: Receiver<TokenEvent>,
    pub snapshot: Receiver<BufferSnapshot>,
    pub word: Receiver<WordEvent>,
}

impl Transport {
    pub fn dropped(&self) -> usize {
        self.dropped.load(Ordering::Relaxed)
    }

    pub fn drain(&mut self, timeline: &mut TimelineData) {
        drain_feed(&mut self.raw, timeline, |t, s| t.push_raw(s));
        drain_feed(&mut self.post_vad, timeline, |t, s| t.push_post_vad(s));
        drain_feed(&mut self.score, timeline, |t, e| t.push_score(e));
        drain_feed(&mut self.chunk, timeline, |t, s| t.push_chunk(s));
        drain_feed(&mut self.token, timeline, |t, e| t.push_token(e));
        drain_feed(&mut self.snapshot, timeline, |t, e| t.push_snapshot(e));
        drain_feed(&mut self.word, timeline, |t, e| t.push_word(e));
    }
}

fn drain_feed<T>(
    receiver: &mut Receiver<T>,
    timeline: &mut TimelineData,
    push: impl Fn(&mut TimelineData, T),
) {
    while let Ok(event) = receiver.try_recv() {
        push(timeline, event);
    }
}
