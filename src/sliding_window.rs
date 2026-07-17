use std::{
    iter::{repeat, repeat_n},
    mem::take,
    time::Duration,
};

use tokio::sync::mpsc::{Sender, error::SendError};

use crate::{
    pipeline::{PipelineConsumer, PipelineProcessor, PipelineProducer},
    timestamped_vec::TimestampedVec,
    units::duration_to_sample_count,
    vad_filter::VadValue,
};

pub struct SlidingWindowChunker {
    window_size: usize,
    cut_left: usize,
    processed_chunk_size: usize,
    buffer: TimestampedVec<f32>,
}

impl SlidingWindowChunker {
    pub fn new(config: &SlidingWindowConfig) -> Self {
        let window_size = duration_to_sample_count(&config.window_size);
        let cut_left = duration_to_sample_count(&config.cut_left);
        let cut_right = duration_to_sample_count(&config.cut_right);
        Self {
            window_size,
            cut_left,
            processed_chunk_size: window_size - cut_left - cut_right,
            buffer: TimestampedVec::default(),
        }
    }
}

impl PipelineConsumer for SlidingWindowChunker {
    type Input = VadValue;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for SlidingWindowChunker {
    type Output = TimestampedVec<f32>;

    fn output_size(&self) -> Option<usize> {
        Some(self.window_size)
    }
}

impl PipelineProcessor for SlidingWindowChunker {
    fn name() -> &'static str {
        "AudioChunker"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        let samples = match value {
            VadValue::Data(samples) => samples,
            VadValue::SpeechEnd => {
                if !self.buffer.is_empty() {
                    let buffer = take(&mut self.buffer);
                    let timestamp = buffer.last().expect("non empty buffer").1;
                    let padding = (0.0, timestamp);
                    let data = buffer
                        .into_iter()
                        .chain(repeat(padding))
                        .take(self.window_size)
                        .collect();
                    sender.send(data).await?;
                    self.buffer.clear();
                }
                return Ok(());
            }
        };

        if self.buffer.is_empty() {
            let timestamp = samples.first().expect("non empty input").1;
            let padding = (0.0, timestamp);
            self.buffer = repeat_n(padding, self.cut_left).collect();
        }

        self.buffer.extend(samples);

        while self.buffer.len() > self.window_size {
            let chunk = self.buffer[..self.window_size].to_vec();
            self.buffer.drain(..self.processed_chunk_size);
            sender.send(chunk).await?;
        }

        Ok(())
    }
}

pub struct SlidingWindowConfig {
    pub window_size: Duration,
    pub cut_left: Duration,
    pub cut_right: Duration,
}
