use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tokio::sync::{Notify, mpsc::Sender};

use crate::{
    pipeline::{PipelineProducer, PipelineSource},
    timestamped_vec::{self, TimestampedVec},
    units::{SAMPLE_DURATION, SAMPLE_RATE_U32, unix_timestamp_now},
};

pub struct AudioCapture;

impl PipelineProducer for AudioCapture {
    type Output = TimestampedVec<f32>;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineSource for AudioCapture {
    fn name() -> &'static str {
        "AudioCapture"
    }

    async fn run(&mut self, sender: Sender<Self::Output>) {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("No input device available");

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: SAMPLE_RATE_U32,
            buffer_size: cpal::BufferSize::Default,
        };

        let exit_notification = Arc::new(Notify::new());
        let exit_notification2 = Arc::clone(&exit_notification);
        let exit_notification3 = Arc::clone(&exit_notification);

        let stream = device
            .build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let timestamp = unix_timestamp_now() - SAMPLE_DURATION * data.len() as u32;

                    // Normalize i16 to f32 [-1.0, 1.0]
                    let normalized: Vec<f32> = data
                        .iter()
                        .map(|&s| s as f32 / (i16::MAX as f32 + 1.0))
                        .collect();

                    sender
                        .blocking_send(timestamped_vec::from_audio_and_timestamp(
                            timestamp, normalized,
                        ))
                        .unwrap_or_else(|_| {
                            exit_notification2.notify_one();
                        });
                },
                move |err| {
                    eprintln!("Audio stream error: {}", err);
                    exit_notification3.notify_one();
                },
                None,
            )
            .expect("Failed to build input stream");

        stream.play().expect("Failed to start audio stream");
        exit_notification.notified().await;
    }
}
