use std::time::Duration;

use crate::units::sample_count_to_duration;

pub type TimestampedVec<T> = Vec<(T, Duration)>;

pub fn from_audio_and_timestamp<T>(timestamp: Duration, audio: Vec<T>) -> TimestampedVec<T> {
    audio
        .into_iter()
        .enumerate()
        .map(|(i, sample)| (sample, timestamp + sample_count_to_duration(i)))
        .collect()
}
