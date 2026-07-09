use std::time::Duration;

use crate::units::SAMPLE_DURATION;

pub type TimestampedVec<T> = Vec<(T, Duration)>;

pub fn from_audio_and_timestamp<T>(timestamp: Duration, audio: Vec<T>) -> TimestampedVec<T> {
    audio
        .into_iter()
        .enumerate()
        .map(|(i, sample)| (sample, timestamp + (i as u32 * SAMPLE_DURATION)))
        .collect()
}
