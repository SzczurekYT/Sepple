use std::{collections::VecDeque, mem::take};

use tokio::sync::mpsc::{Sender, error::SendError};

use crate::{
    pipeline::{PipelineConsumer, PipelineProcessor, PipelineProducer},
    timestamped_vec::TimestampedVec,
};

pub struct VadFilter {
    start_talk_threshold: f32,
    end_talk_threshold: f32,
    previous: Option<(TimestampedVec<f32>, f32)>,
    context: VecDeque<(TimestampedVec<f32>, f32)>,
    context_size_chunks: usize,
    is_talking: bool,
}

impl VadFilter {
    pub fn new(
        start_talk_threshold: f32,
        end_talk_threshold: f32,
        context_size_chunks: usize,
    ) -> Self {
        Self {
            start_talk_threshold,
            end_talk_threshold,
            previous: None,
            context: VecDeque::new(),
            context_size_chunks,
            is_talking: false,
        }
    }

    fn add_to_context(&mut self, chunk: (TimestampedVec<f32>, f32)) {
        if self.context.len() >= self.context_size_chunks {
            self.context.pop_front();
        }
        self.context.push_back(chunk);
    }

    fn should_start_talking(&mut self, score: f32, previous_score: f32) -> bool {
        previous_score >= self.start_talk_threshold
            && score >= self.start_talk_threshold
            && score >= previous_score
    }

    fn should_stop_talking(&mut self, score: f32, previous_score: f32) -> bool {
        previous_score < self.end_talk_threshold
            && score < self.end_talk_threshold
            && score <= previous_score
    }
}

impl PipelineConsumer for VadFilter {
    type Input = (TimestampedVec<f32>, f32);

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for VadFilter {
    type Output = VadValue;

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
            if self.should_start_talking(score, previous_score) {
                // Clear context and send all recent audio
                for (chunk, _) in take(&mut self.context) {
                    sender.send(VadValue::Data(chunk)).await?;
                }
                self.is_talking = true;
            } else {
                self.add_to_context((audio.clone(), score));
            }
        } else {
            if self.should_stop_talking(score, previous_score) {
                self.is_talking = false;
                sender.send(VadValue::SpeechEnd).await?;
            } else {
                sender.send(VadValue::Data(previous)).await?;
            }
        }

        self.previous = Some((audio, score));

        Ok(())
    }
}

pub enum VadValue {
    Data(TimestampedVec<f32>),
    SpeechEnd,
}
