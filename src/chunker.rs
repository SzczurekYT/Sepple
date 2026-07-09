use tokio::sync::mpsc::{Sender, error::SendError};

use crate::{
    pipeline::{PipelineConsumer, PipelineProcessor, PipelineProducer},
    timestamped_vec::TimestampedVec,
};

pub struct AudioChunker {
    chunk_size: usize,
    buffer: TimestampedVec<f32>,
}

impl AudioChunker {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            buffer: TimestampedVec::default(),
        }
    }
}

impl PipelineConsumer for AudioChunker {
    type Input = TimestampedVec<f32>;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for AudioChunker {
    type Output = TimestampedVec<f32>;

    fn output_size(&self) -> Option<usize> {
        Some(self.chunk_size)
    }
}

impl PipelineProcessor for AudioChunker {
    fn name() -> &'static str {
        "AudioChunker"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        self.buffer.extend(value);

        if self.buffer.len() < self.chunk_size {
            return Ok(());
        }

        let mut iter = self.buffer.chunks_exact(self.chunk_size);

        for chunk in iter.by_ref() {
            sender.send(chunk.to_vec()).await?;
        }
        self.buffer = iter.remainder().to_vec();

        Ok(())
    }
}
