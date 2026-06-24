use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::{
    ipapipeline::{SAMPLE_RATE_F32, SAMPLE_RATE_U32},
    util::unix_timestamp_now,
};

pub struct CapturedAudio {
    pub timestamp: u128,
    pub audio: Vec<f32>,
}

pub fn start_audio_capture(chunk_length: Duration) -> Receiver<CapturedAudio> {
    let (tx, rx) = mpsc::channel::<CapturedAudio>();

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

    let chunk_size = (chunk_length.as_secs_f32() * SAMPLE_RATE_F32) as usize;

    let stream = device
        .build_input_stream(
            &config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                let timestamp = unix_timestamp_now();
                // Normalize i16 to f32 [-1.0, 1.0]
                let normalized: Vec<f32> = data
                    .iter()
                    .map(|&s| s as f32 / (i16::MAX as f32 + 1.0))
                    .collect();

                buffer.extend_from_slice(&normalized);

                while buffer.len() >= chunk_size {
                    let chunk: Vec<f32> = buffer.drain(..chunk_size).collect();
                    if tx
                        .send(CapturedAudio {
                            timestamp,
                            audio: chunk,
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )
        .expect("Failed to build input stream");

    thread::spawn(move || {
        stream.play().expect("Failed to start audio stream");
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    rx
}
