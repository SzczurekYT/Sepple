use std::{
    fmt::{self, Display},
    sync::Arc,
    time::Duration,
};

use burn::backend::Flex;
use tokio::{
    sync::{
        Notify,
        mpsc::{Sender, error::SendError},
    },
    task::spawn_blocking,
};

use crate::{
    ipa_recognizer::{IpaRecognizer, z_score_normalize},
    pipeline::{PipelineConsumer, PipelineProcessor, PipelineProducer},
    sliding_window::SlidingWindowConfig,
    timestamped_vec::TimestampedVec,
    units::{duration_to_logit_count, duration_to_sample_count, logit_count_to_time},
};

pub struct IpaProcessor {
    recognizer: Arc<IpaRecognizer<Flex>>,
    window_size: usize,
    notification: Arc<Notify>,
    cut_left_logits: usize,
    cut_right_logits: usize,
}

impl IpaProcessor {
    pub fn init(config: &SlidingWindowConfig) -> Self {
        let recognizer = IpaRecognizer::init().into();
        let notification = Arc::new(Notify::new());
        notification.notify_one();
        Self {
            recognizer,
            notification,
            window_size: duration_to_sample_count(&config.window_size),
            cut_left_logits: duration_to_logit_count(config.cut_left),
            cut_right_logits: duration_to_logit_count(config.cut_right),
        }
    }

    fn next_notification(&mut self) -> (Arc<Notify>, Arc<Notify>) {
        let old = Arc::clone(&self.notification);
        let new = Arc::new(Notify::new());
        self.notification = Arc::clone(&new);
        (old, new)
    }
}

impl PipelineConsumer for IpaProcessor {
    type Input = TimestampedVec<f32>;

    fn input_size(&self) -> Option<usize> {
        Some(self.window_size)
    }
}

impl PipelineProducer for IpaProcessor {
    type Output = TimestampedText;

    fn output_size(&self) -> Option<usize> {
        None
    }
}

impl PipelineProcessor for IpaProcessor {
    fn name() -> &'static str {
        "PipelineProcessor"
    }

    async fn process_value(
        &mut self,
        value: Self::Input,
        sender: &Sender<Self::Output>,
    ) -> Result<(), SendError<Self::Output>> {
        let mut start_time = value.first().expect("non empty input").1;
        let sample_iter = value.iter().map(|(sample, _)| *sample);
        let chunk = z_score_normalize(sample_iter);

        let recognizer = Arc::clone(&self.recognizer);
        let (notif, next_notif) = self.next_notification();

        let logits = spawn_blocking(move || recognizer.process(&chunk))
            .await
            .unwrap();

        notif.notified().await;

        next_notif.notify_one();

        let cut_logits = &logits[self.cut_left_logits..(logits.len() - self.cut_right_logits)];

        start_time += logit_count_to_time(self.cut_left_logits);

        let last_letter_index = cut_logits
            .iter()
            .rev()
            .position(|logit| *logit != self.recognizer.padding_token_id())
            .map(|index| logits.len() - 1 - index)
            .unwrap_or(0);

        let end_time = start_time + logit_count_to_time(last_letter_index);

        let ctc_decoded = self.recognizer.greedy_ctc_decode(cut_logits);
        let text = self.recognizer.decode_tokens(&ctc_decoded);

        if text.is_empty() {
            return Ok(());
        }

        sender
            .send(TimestampedText {
                text,
                start_time,
                end_time,
            })
            .await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TimestampedText {
    pub text: String,
    pub start_time: Duration,
    pub end_time: Duration,
}

impl Display for TimestampedText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Text: \"{}\" at {:?}, duration: {:?}",
            self.text,
            self.start_time,
            self.end_time - self.start_time
        )
    }
}
