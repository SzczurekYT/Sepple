use std::time::Duration;

use tokio::sync::mpsc::{Sender, error::SendError};

use crate::{
    pipeline::{PipelineConsumer, PipelineProcessor, PipelineProducer},
    timestamped_vec::TimestampedVec,
    units::duration_to_sample_count,
};

pub struct SlidingWindowChunker {
    window_size: usize,
    processed_chunk_size: usize,
    buffer: TimestampedVec<f32>,
}

impl SlidingWindowChunker {
    pub fn new(config: &SlidingWindowConfig) -> Self {
        let window_size = duration_to_sample_count(&config.window_size);
        Self {
            window_size,
            processed_chunk_size: window_size
                - duration_to_sample_count(&config.cut_left)
                - duration_to_sample_count(&config.cut_right),
            buffer: TimestampedVec::default(),
        }
    }
}

impl PipelineConsumer for SlidingWindowChunker {
    type Input = TimestampedVec<f32>;

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
        self.buffer.extend(value);

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
