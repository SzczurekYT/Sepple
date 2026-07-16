use std::path::PathBuf;

use tokio::sync::mpsc::{Sender, error::SendError};

use crate::{
    pipeline::{PipelineConsumer, PipelineProcessor, PipelineProducer},
    save_f32_to_wav,
    timestamped_vec::TimestampedVec,
};

pub struct AudioLogger {
    chunk_size: Option<usize>,
    path: PathBuf,
    counter: usize,
}

impl AudioLogger {
    pub fn new(chunk_size: Option<usize>, path: &str) -> Self {
        Self {
            chunk_size,
            path: path.into(),
            counter: 0,
        }
    }
}

impl PipelineConsumer for AudioLogger {
    type Input = TimestampedVec<f32>;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for AudioLogger {
    type Output = TimestampedVec<f32>;

    fn output_size(&self) -> Option<usize> {
        self.chunk_size
    }
}

impl PipelineProcessor for AudioLogger {
    fn name() -> &'static str {
        "AudioLogger"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        let audio: Vec<f32> = value.iter().map(|(sample, _)| *sample).collect();

        let path = self.path.join(format!("debug_{}.wav", self.counter));
        self.counter += 1;
        save_f32_to_wav(&audio, path);

        sender.send(value).await?;

        Ok(())
    }
}
