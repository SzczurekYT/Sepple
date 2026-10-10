use std::{
    fmt::{self, Display},
    ops::Range,
    time::Duration,
};

use tokio::sync::mpsc::{Sender, error::SendError};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    debug_enabled,
    dictionary::Dictionary,
    pipeline::{
        PipelineConsumer, PipelineProcessor, PipelineProducer,
        processor::ipa_processor::TimestampedText,
    },
    units::{logit_count_to_time, unix_timestamp_now},
};

const TIME_DIFFERENCE_CUTOFF: Duration = Duration::from_millis(300);

#[derive(Debug, Clone)]
pub struct DetectedWord {
    pub word: String,
    pub audio_span: Range<Duration>,
    pub detected_at: Duration,
}

impl Display for DetectedWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.word.fmt(f)
    }
}

pub struct WordDetector {
    text_buffer: String,
    timestamps: Vec<Duration>,
    dictionary: Dictionary,
    last_end_time: Duration,
}

impl WordDetector {
    pub fn init(dictionary: Dictionary) -> Self {
        Self {
            text_buffer: String::with_capacity(100),
            timestamps: Vec::new(),
            dictionary,
            last_end_time: Duration::ZERO,
        }
    }

    pub fn buffer(&self) -> &str {
        &self.text_buffer
    }
}

impl PipelineConsumer for WordDetector {
    type Input = TimestampedText;

    fn input_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProducer for WordDetector {
    type Output = DetectedWord;

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
        sender: &Sender<DetectedWord>,
    ) -> Result<(), SendError<DetectedWord>> {
        let start_time = value.start_time();
        let end_time = value.end_time();
        let TimestampedText { text, timestamps } = value;

        if start_time > self.last_end_time
            && start_time - self.last_end_time > TIME_DIFFERENCE_CUTOFF
        {
            self.text_buffer.clear();
            self.timestamps.clear();
        }
        self.last_end_time = end_time;

        self.text_buffer.push_str(&text);
        self.timestamps.extend(timestamps);

        if debug_enabled() {
            println!("WD Received: {text}");
            println!("WD Buffer: {}", self.text_buffer);
        }

        let (words, consumed) = self
            .dictionary
            .find_words_in_string_indexed(&self.text_buffer);

        let token_duration = logit_count_to_time(1);
        for (word, start_index, end_index) in words {
            let start_grapheme_index = self.text_buffer[..start_index].graphemes(true).count();
            let end_grapheme_index = start_grapheme_index
                + self.text_buffer[start_index..end_index]
                    .graphemes(true)
                    .count();
            let audio_span = self.timestamps[start_grapheme_index]
                ..(self.timestamps[end_grapheme_index - 1] + token_duration);

            sender
                .send(DetectedWord {
                    word: word.to_owned(),
                    audio_span,
                    detected_at: unix_timestamp_now(),
                })
                .await?;
        }

        if consumed != 0 {
            let removed = self.text_buffer[..consumed].graphemes(true).count();
            self.text_buffer = self.text_buffer[consumed..].to_owned();
            self.timestamps.drain(..removed);
        } else {
            let limit = self.dictionary.longest_considered_word_len;
            let mut iterator = self.text_buffer.grapheme_indices(true).rev();
            let first_kept_grapheme = iterator.nth(limit);
            let grapheme_over_limit = iterator.next();
            if grapheme_over_limit.is_some() {
                let (index, _) = first_kept_grapheme
                    .expect("some, because previous element in iterator was some");
                let removed = self.text_buffer[..index].graphemes(true).count();
                self.text_buffer = self.text_buffer[index..].to_owned();
                self.timestamps.drain(..removed);
            }
        }

        if debug_enabled() {
            println!("WD Buffer After: {}", self.text_buffer);
        }

        Ok(())
    }
}
