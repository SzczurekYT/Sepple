use std::{
    sync::mpsc::{Receiver, TryRecvError},
    thread,
    time::Duration,
};

use burn::backend::Flex;

use crate::iparecognizer::{IpaRecognizer, z_score_normalize};

pub const SAMPLE_RATE_U32: u32 = 16_000;
pub const SAMPLE_RATE_USIZE: usize = 16_000;
pub const SAMPLE_RATE_F32: f32 = 16_000.0;
pub const DOWNSAMPLE_RATE_F32: f32 = 320.0;
pub const TIME_TO_LOGIT_FACTOR: f32 = SAMPLE_RATE_F32 / DOWNSAMPLE_RATE_F32;

pub struct IpaPipeline {
    recognizer: IpaRecognizer<Flex>,
    config: SlidingWindowConfig,
    buffer: Vec<f32>,
    token_buffer: Vec<i32>,
}

impl IpaPipeline {
    pub fn init(config: SlidingWindowConfig) -> Self {
        let recognizer = IpaRecognizer::init();
        Self {
            recognizer,

            buffer: Vec::with_capacity(
                (config.window_size.as_secs() + 1) as usize * SAMPLE_RATE_USIZE,
            ),
            token_buffer: Vec::with_capacity(200),
            config,
        }
    }

    pub fn run(&mut self, receiver: Receiver<Vec<f32>>) {
        let window_sample_count =
            (SAMPLE_RATE_F32 * self.config.window_size.as_secs_f32()).round() as usize;
        let stride_sample_count =
            (SAMPLE_RATE_F32 * self.config.stride.as_secs_f32()).round() as usize;
        let stride_logit_count = time_to_logit_count(self.config.stride);
        let processed_chunk_sample_count = window_sample_count - 2 * stride_sample_count;
        loop {
            let data = match receiver.try_recv() {
                Ok(data) => data,
                Err(TryRecvError::Disconnected) => {
                    let ctc_decoded = self.recognizer.greedy_ctc_decode(&self.token_buffer);
                    println!(
                        "Pipeline finished, text in buffer: {}",
                        self.recognizer.decode_tokens(&ctc_decoded)
                    );
                    break;
                }
                Err(TryRecvError::Empty) => {
                    thread::sleep(Duration::from_millis(500));
                    continue;
                }
            };
            self.buffer.extend(data);

            while self.buffer.len() > window_sample_count {
                let buf = &self.buffer[..window_sample_count];
                let buf = &z_score_normalize(buf);
                let logits = self.recognizer.process(buf);
                self.buffer.drain(..processed_chunk_sample_count);
                let cut_logits = &logits[stride_logit_count..(logits.len() - stride_logit_count)];
                self.token_buffer.extend(cut_logits);
                let ctc_decoded = self.recognizer.greedy_ctc_decode(cut_logits);
                println!("Text: {}", self.recognizer.decode_tokens(&ctc_decoded))
            }
        }
    }
}

pub struct SlidingWindowConfig {
    pub window_size: Duration,
    pub stride: Duration,
}

fn time_to_logit_count(time: Duration) -> usize {
    (TIME_TO_LOGIT_FACTOR * time.as_secs_f32()) as usize
}
