use tokio::sync::mpsc::Sender;

use crate::{
    pipeline::{PipelineProducer, PipelineSource},
    timestamped_vec::{self, TimestampedVec},
    units::unix_timestamp_now,
};

pub struct MemoryAudioSource {
    data: Vec<f32>,
}

impl MemoryAudioSource {
    pub fn new(data: Vec<f32>) -> Self {
        Self { data }
    }
}

impl PipelineProducer for MemoryAudioSource {
    type Output = TimestampedVec<f32>;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineSource for MemoryAudioSource {
    fn name() -> &'static str {
        "MemoryAudioSource"
    }

    async fn run(&mut self, sender: Sender<Self::Output>) {
        let now = unix_timestamp_now();
        let data = timestamped_vec::from_audio_and_timestamp(now, self.data.clone());
        sender.send(data).await.ok();
    }
}
