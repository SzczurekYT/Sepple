use std::sync::mpsc::{self, Receiver};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::ipapipeline::SAMPLE_RATE_U32;

pub fn start_audio_capture() -> Receiver<Vec<f32>> {
    let (tx, rx) = mpsc::channel::<Vec<f32>>();

    let mut buffer = Vec::<f32>::new();

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");

    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: SAMPLE_RATE_U32,
        buffer_size: cpal::BufferSize::Default,
    };

    let stream = device
        .build_input_stream(
            &config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                // Normalize i16 to f32 [-1.0, 1.0]
                let normalized: Vec<f32> = data
                    .iter()
                    .map(|&s| s as f32 / (i16::MAX as f32 + 1.0))
                    .collect();

                buffer.extend_from_slice(&normalized);

                while buffer.len() >= 16_000 {
                    let chunk: Vec<f32> = buffer.drain(..16_000).collect();
                    if tx.send(chunk).is_err() {
                        break;
                    }
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )
        .expect("Failed to build input stream");

    std::thread::spawn(move || {
        stream.play().expect("Failed to start audio stream");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    rx
}
