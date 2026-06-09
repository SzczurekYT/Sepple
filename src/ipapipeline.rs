use std::{
    sync::mpsc::{Receiver, TryRecvError},
    thread,
    time::Duration,
};

use burn::backend::Flex;

use crate::iparecognizer::IpaRecognizer;

pub const SAMPLE_RATE_U32: u32 = 16_000;
pub const SAMPLE_RATE_USIZE: usize = 16_000;

pub struct IpaPipeline {
    recognizer: IpaRecognizer<Flex>,
    receiver: Receiver<Vec<f32>>,
    buffer: Vec<f32>,
    token_buffer: Vec<i32>,
}

impl IpaPipeline {
    pub fn init(receiver: Receiver<Vec<f32>>) -> Self {
        let recognizer = IpaRecognizer::init();
        Self {
            recognizer,
            receiver,
            buffer: Vec::with_capacity(3 * SAMPLE_RATE_USIZE),
            token_buffer: Vec::with_capacity(200),
        }
    }

    pub fn run(&mut self) {
        loop {
            let data = match self.receiver.try_recv() {
                Ok(data) => data,
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {
                    thread::sleep(Duration::from_millis(500));
                    continue;
                }
            };
            println!("Got {} of data", data.len());
            self.buffer.extend(data);

            let sample_count = SAMPLE_RATE_USIZE / 4 * 3;

            if self.buffer.len() > sample_count {
                let buf = &self.buffer[..sample_count];
                let result = self.recognizer.process(buf);
                self.buffer.drain(..sample_count);
                let result = self.recognizer.greedy_ctc_decode(&result);
                println!("Text: {}", self.recognizer.decode_tokens(&result))
            }
        }
    }
}
