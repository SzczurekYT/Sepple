use tokio::sync::mpsc::{Sender, error::SendError};

use crate::{
    pipeline::{PipelineConsumer, PipelineProcessor, PipelineProducer},
    timestamped_vec::TimestampedVec,
};

pub struct VadFilter {
    start_talk_threshold: f32,
    end_talk_threshold: f32,
    previous: Option<(TimestampedVec<f32>, f32)>,
    is_talking: bool,
}

impl VadFilter {
    pub fn new(start_talk_threshold: f32, end_talk_threshold: f32) -> Self {
        Self {
            start_talk_threshold,
            end_talk_threshold,
            previous: None,
            is_talking: false,
        }
    }
}

impl PipelineConsumer for VadFilter {
    type Input = (TimestampedVec<f32>, f32);

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for VadFilter {
    type Output = TimestampedVec<f32>;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProcessor for VadFilter {
    fn name() -> &'static str {
        "VadFilter"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        let (audio, score) = value;

        let Some((previous, previous_score)) = self.previous.take() else {
            self.previous = Some((audio, score));
            return Ok(());
        };

        if !self.is_talking {
            if previous_score >= self.start_talk_threshold
                && score >= self.start_talk_threshold
                && score >= previous_score
            {
                sender.send(previous).await?;
                self.is_talking = true;
            }
        } else {
            if previous_score < self.end_talk_threshold
                && score < self.end_talk_threshold
                && score <= previous_score
            {
                self.is_talking = false;
            } else {
                sender.send(previous).await?;
            }
        }

        self.previous = Some((audio, score));

        Ok(())
    }
}
