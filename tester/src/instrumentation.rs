use std::{
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};

use sepple::{
    pipeline::{
        PipelineConsumer, PipelineProcessor, PipelineProducer, PipelineSink,
        processor::{
            ipa_processor::TimestampedText,
            vad_filter::{VadFilter, VadValue},
            word_detector::{DetectedWord, WordDetector},
        },
    },
    timestamped_vec::TimestampedVec,
    units::{sample_count_to_duration, unix_timestamp_now},
};
use tokio::sync::mpsc::{self, Receiver, Sender, error::SendError};

use crate::gui::timeline_data::{BufferSnapshot, VadScoreEvent};

const CHANNEL_BUFFER_SIZE: usize = 50;

pub struct Tap<T> {
    output_size: Option<usize>,
    sender: Sender<T>,
    dropped: Arc<AtomicUsize>,
}

impl<T> Tap<T> {
    pub fn new(dropped: Arc<AtomicUsize>, output_size: Option<usize>) -> (Self, Receiver<T>) {
        let (sender, receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        (
            Self {
                output_size,
                sender,
                dropped,
            },
            receiver,
        )
    }
}

impl<T: Clone> Tap<T> {
    fn push(&self, value: &T) {
        if self.sender.try_send(value.clone()).is_err() {
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl<T: Send + 'static> PipelineConsumer for Tap<T> {
    type Input = T;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl<T: Send + 'static> PipelineProducer for Tap<T> {
    type Output = T;

    fn output_size(&self) -> Option<usize> {
        self.output_size
    }
}

impl<T: Send + Clone + 'static> PipelineProcessor for Tap<T> {
    fn name() -> &'static str {
        "Tap"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        self.push(&value);
        sender.send(value).await
    }
}

impl<T: Send + Clone + 'static> PipelineSink for Tap<T> {
    fn name() -> &'static str {
        "Tap"
    }

    async fn consume_value(&mut self, value: Self::Input) {
        self.push(&value);
    }
}

pub struct SilenceGate {
    running: Arc<AtomicBool>,
}

impl SilenceGate {
    pub fn new(running: Arc<AtomicBool>) -> Self {
        Self { running }
    }
}

impl PipelineConsumer for SilenceGate {
    type Input = TimestampedVec<f32>;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for SilenceGate {
    type Output = TimestampedVec<f32>;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProcessor for SilenceGate {
    fn name() -> &'static str {
        "SilenceGate"
    }

    async fn process_value(
        &mut self,
        mut value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        if !self.running.load(Ordering::Relaxed) {
            for (sample, _) in value.iter_mut() {
                *sample = 0.0;
            }
        }
        sender.send(value).await
    }
}

pub struct InstrumentedVadFilter {
    inner: VadFilter,
    score_feed: Sender<VadScoreEvent>,
    dropped: Arc<AtomicUsize>,
}

impl InstrumentedVadFilter {
    pub fn new(inner: VadFilter, dropped: Arc<AtomicUsize>) -> (Self, Receiver<VadScoreEvent>) {
        let (sender, receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        (
            Self {
                inner,
                score_feed: sender,
                dropped,
            },
            receiver,
        )
    }
}

impl PipelineConsumer for InstrumentedVadFilter {
    type Input = (TimestampedVec<f32>, f32);

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for InstrumentedVadFilter {
    type Output = VadValue;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProcessor for InstrumentedVadFilter {
    fn name() -> &'static str {
        "InstrumentedVadFilter"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        let (audio, score) = value;
        let span = audio_span(&audio);
        let result = self.inner.process_value((audio, score), sender).await;
        let event = VadScoreEvent {
            span,
            score,
            gate_open: self.inner.is_talking(),
        };
        if self.score_feed.try_send(event).is_err() {
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
        result
    }
}

pub struct InstrumentedWordDetector {
    inner: WordDetector,
    snapshot_feed: Sender<BufferSnapshot>,
    dropped: Arc<AtomicUsize>,
    last_buffer: String,
}

impl InstrumentedWordDetector {
    pub fn new(inner: WordDetector, dropped: Arc<AtomicUsize>) -> (Self, Receiver<BufferSnapshot>) {
        let (sender, receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        (
            Self {
                inner,
                snapshot_feed: sender,
                dropped,
                last_buffer: String::new(),
            },
            receiver,
        )
    }
}

impl PipelineConsumer for InstrumentedWordDetector {
    type Input = TimestampedText;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for InstrumentedWordDetector {
    type Output = DetectedWord;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProcessor for InstrumentedWordDetector {
    fn name() -> &'static str {
        "InstrumentedWordDetector"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        let result = self.inner.process_value(value, sender).await;
        let buffer = self.inner.buffer();
        if buffer != self.last_buffer {
            let snapshot = BufferSnapshot {
                at: unix_timestamp_now(),
                text: buffer.to_owned(),
            };
            if self.snapshot_feed.try_send(snapshot).is_err() {
                self.dropped.fetch_add(1, Ordering::Relaxed);
            }
            self.last_buffer = buffer.to_owned();
        }
        result
    }
}

fn audio_span(audio: &TimestampedVec<f32>) -> Range<Duration> {
    let (_, first) = audio.first().expect("non empty TimestampedVec");
    let (_, last) = audio.last().expect("non empty TimestampedVec");
    *first..(*last + sample_count_to_duration(1))
}
