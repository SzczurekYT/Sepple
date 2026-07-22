use std::time::Duration;

use tokio::sync::mpsc::{Sender, error::SendError};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    debug_enabled,
    dictionary::Dictionary,
    pipeline::{
        PipelineConsumer, PipelineProcessor, PipelineProducer,
        processor::ipa_processor::TimestampedText,
    },
};

const TIME_DIFFERENCE_CUTOFF: Duration = Duration::from_millis(300);

pub struct WordDetector {
    text_buffer: String,
    dictionary: Dictionary,
    last_end_time: Duration,
}

impl WordDetector {
    pub fn init(confusion_distance_threshold: f64) -> Self {
        Self {
            text_buffer: String::with_capacity(100),
            dictionary: Dictionary::load(confusion_distance_threshold),
            last_end_time: Duration::ZERO,
        }
    }
}

impl PipelineConsumer for WordDetector {
    type Input = TimestampedText;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for WordDetector {
    type Output = String;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProcessor for WordDetector {
    fn name() -> &'static str {
        "WordDetector"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<std::string::String>,
    ) -> Result<(), SendError<Self::Output>> {
        let TimestampedText {
            text,
            start_time,
            end_time,
        } = value;

        if start_time > self.last_end_time
            && start_time - self.last_end_time > TIME_DIFFERENCE_CUTOFF
        {
            self.text_buffer.clear();
        }
        self.last_end_time = end_time;

        self.text_buffer.push_str(&text);
        if debug_enabled() {
            println!("WD Received: {text}");
            println!("WD Buffer: {}", self.text_buffer);
        }

        let (words, consumed) = self.dictionary.find_words_in_string(&self.text_buffer);

        for word in words {
            sender.send(word.to_owned()).await?;
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
        if debug_enabled() {
            println!("WD Buffer After: {}", self.text_buffer);
        }

        Ok(())
    }
}
