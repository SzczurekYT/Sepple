use std::{
    sync::{
        Arc,
        mpsc::{self, Receiver, Sender, TryRecvError},
    },
    thread,
    time::Duration,
};

use burn::backend::Flex;

use crate::{
    capture::CapturedAudio,
    iparecognizer::{IpaRecognizer, z_score_normalize},
};

pub const SAMPLE_RATE_U32: u32 = 16_000;
pub const SAMPLE_RATE_USIZE: usize = 16_000;
pub const SAMPLE_RATE_F32: f32 = 16_000.0;
pub const DOWNSAMPLE_RATE_F32: f32 = 320.0;
pub const LOGITS_PER_SECOND: f32 = SAMPLE_RATE_F32 / DOWNSAMPLE_RATE_F32;

pub struct IpaPipeline {
    recognizer: Arc<IpaRecognizer<Flex>>,
    config: SlidingWindowConfig,
    buffer: Vec<f32>,
}

impl IpaPipeline {
    pub fn init(config: SlidingWindowConfig) -> Self {
        let recognizer = IpaRecognizer::init().into();
        Self {
            recognizer,
            buffer: Vec::with_capacity(
                (config.window_size.as_secs() + 1) as usize * SAMPLE_RATE_USIZE,
            ),
            config,
        }
    }

    pub fn run(&mut self, receiver: Receiver<CapturedAudio>, result_sender: Sender<PipelineValue>) {
        let mut values_buffer = Vec::with_capacity(4);

        let mut next_task_id: u32 = 0;
        let mut next_finished_task_id: u32 = 0;

        let (thread_sender, thread_receiver) = mpsc::channel();

        let padding_token_id = self.recognizer.padding_token_id();

        let window_sample_count =
            (SAMPLE_RATE_F32 * self.config.window_size.as_secs_f32()).round() as usize;
        let stride_sample_count =
            (SAMPLE_RATE_F32 * self.config.stride.as_secs_f32()).round() as usize;
        let stride_logit_count = time_to_logit_count(self.config.stride);
        let processed_chunk_sample_count = window_sample_count - 2 * stride_sample_count;

        thread::scope(|s| {
            'main_loop: loop {
                let CapturedAudio { timestamp, audio } = match receiver.try_recv() {
                    Ok(data) => data,
                    Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }
                };
                self.buffer.extend(audio);

                while self.buffer.len() > window_sample_count {
                    let buf = &self.buffer[..window_sample_count];
                    let buf = z_score_normalize(buf);
                    self.buffer.drain(..processed_chunk_sample_count);
                    let task_id = next_task_id;
                    next_task_id += 1;

                    let recognizer = Arc::clone(&self.recognizer);
                    let thread_sender = thread_sender.clone();

                    s.spawn(move || {
                        let logits = recognizer.process(&buf);
                        let _ = thread_sender.send((task_id, logits));
                    });
                }

                while let Ok(value) = thread_receiver.try_recv() {
                    values_buffer.push(value);
                }

                while let Some(index) = values_buffer
                    .iter()
                    .position(|(id, _)| *id == next_finished_task_id)
                {
                    let (_, logits) = values_buffer.swap_remove(index);
                    let cut_logits =
                        &logits[stride_logit_count..(logits.len() - stride_logit_count)];

                    let last_letter_index = cut_logits
                        .iter()
                        .rev()
                        .position(|logit| *logit != padding_token_id)
                        .map(|value| logits.len() - value - 1)
                        .unwrap_or(0);

                    let end_time = timestamp + logit_count_to_time(last_letter_index).as_millis();

                    let ctc_decoded = self.recognizer.greedy_ctc_decode(cut_logits);
                    let text = self.recognizer.decode_tokens(&ctc_decoded);

                    let has_new_text = !text.is_empty();

                    if has_new_text {
                        let send_result = result_sender.send(PipelineValue {
                            text,
                            start_time: timestamp,
                            end_time,
                        });

                        if send_result.is_err() {
                            break 'main_loop;
                        }
                    }

                    next_finished_task_id += 1;
                }
            }
        });
    }
}

pub struct SlidingWindowConfig {
    pub window_size: Duration,
    pub stride: Duration,
}

fn time_to_logit_count(time: Duration) -> usize {
    (time.as_secs_f32() * LOGITS_PER_SECOND) as usize
}

fn logit_count_to_time(count: usize) -> Duration {
    Duration::from_secs_f32(count as f32 / LOGITS_PER_SECOND)
}

#[derive(Debug, Clone)]
pub struct PipelineValue {
    pub text: String,
    pub start_time: u128,
    pub end_time: u128,
}
