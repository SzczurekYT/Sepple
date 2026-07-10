use tokio::sync::mpsc::{Sender, error::SendError};

use crate::{
    SeppleBackend,
    pipeline::{PipelineConsumer, PipelineProcessor, PipelineProducer},
    timestamped_vec::TimestampedVec,
    vad::{self, Vad},
};

pub struct SileroVadScorer {
    vad: Vad<SeppleBackend>,
}

impl SileroVadScorer {
    pub fn init() -> Self {
        Self { vad: Vad::init() }
    }
}

impl PipelineConsumer for SileroVadScorer {
    type Input = TimestampedVec<f32>;

    fn input_size(&self) -> Option<usize> {
        Some(vad::CHUNK_SIZE)
    }
}

impl PipelineProducer for SileroVadScorer {
    type Output = (TimestampedVec<f32>, f32);

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProcessor for SileroVadScorer {
    fn name() -> &'static str {
        "SileroVadScorer"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        let audio: Vec<f32> = value.iter().map(|(sample, _)| *sample).collect();

        let score = self.vad.process_chunk(&audio);
        sender.send((value, score)).await?;

        Ok(())
    }
}
