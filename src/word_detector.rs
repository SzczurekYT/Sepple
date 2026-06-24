use std::{
    ops::ControlFlow,
    sync::mpsc::{Receiver, Sender, TryRecvError},
    thread,
    time::Duration,
};

use unicode_segmentation::UnicodeSegmentation;

use crate::{dictionary::Dictionary, ipapipeline::PipelineValue};

const TIME_DIFFERENCE_CUTOFF_MS: u128 = 300;

pub struct WordDetector {
    text_buffer: String,
    dictionary: Dictionary,
    last_end_time: u128,
}

impl WordDetector {
    pub fn init() -> Self {
        Self {
            text_buffer: String::with_capacity(100),
            dictionary: Dictionary::load(),
            last_end_time: u128::MAX,
        }
    }

    pub fn run(&mut self, receiver: Receiver<PipelineValue>, result_sender: Sender<String>) {
        loop {
            let data = match receiver.try_recv() {
                Ok(data) => data,
                Err(TryRecvError::Disconnected) => {
                    break;
                }
                Err(TryRecvError::Empty) => {
                    thread::sleep(Duration::from_millis(500));
                    continue;
                }
            };
            let result = self.on_new_data(data, &result_sender);
            if result.is_break() {
                break;
            }
        }
    }

    pub fn on_new_data(
        &mut self,
        data: PipelineValue,
        result_sender: &Sender<String>,
    ) -> ControlFlow<()> {
        let PipelineValue {
            text,
            start_time,
            end_time,
        } = data;

        if start_time - self.last_end_time > TIME_DIFFERENCE_CUTOFF_MS {
            self.text_buffer.clear();
        }
        self.last_end_time = end_time;

        self.text_buffer.push_str(&text);
        println!("Text {}", self.text_buffer);

        let (words, consumed) = self.dictionary.greedy_search(&self.text_buffer);
        for word in words {
            let send_result = result_sender.send(word.to_owned());
            if send_result.is_err() {
                return ControlFlow::Break(());
            }
        }
        if consumed != 0 {
            self.text_buffer = self.text_buffer[consumed..].to_owned();
        } else {
            let limit = self.dictionary.longest_considered_word_len;
            let mut iterator = self.text_buffer.grapheme_indices(true).rev();
            let first_kept_grapheme = iterator.nth(limit);
            let grapheme_over_limit = iterator.next();
            if grapheme_over_limit.is_some() {
                let (index, _) = first_kept_grapheme
                    .expect("some, because previous element in iterator was some");
                self.text_buffer = self.text_buffer[index..].to_owned();
            }
        }
        ControlFlow::Continue(())
    }
}
