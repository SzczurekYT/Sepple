use std::{mem, time::Instant};

use tokio::{sync::mpsc::Sender, time::sleep};

use crate::{
    pipeline::{PipelineProducer, PipelineSource},
    timestamped_vec::{self, TimestampedVec},
    units::{sample_count_to_duration, unix_timestamp_now},
};

pub struct MemoryAudioSource {
    data: Vec<f32>,
    chunk_size: usize,
}

impl MemoryAudioSource {
    pub fn new(data: Vec<f32>, chunk_size: usize) -> Self {
        Self { data, chunk_size }
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
        let data = mem::take(&mut self.data);

        let chunk_duration = sample_count_to_duration(self.chunk_size);

        let start_instant = Instant::now();
        for (i, chunk) in data.chunks(self.chunk_size).enumerate() {
            let target_elapsed = chunk_duration * i as u32;
            let elapsed = start_instant.elapsed();

            if target_elapsed > elapsed {
                sleep(target_elapsed - elapsed).await;
            }

            let timestamped =
                timestamped_vec::from_audio_and_timestamp(unix_timestamp_now(), chunk.to_vec());

            if sender.send(timestamped).await.is_err() {
                return;
            }
        }
    }
}
