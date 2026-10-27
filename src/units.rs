use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const SAMPLE_RATE_U32: u32 = 16_000;
pub const SAMPLE_RATE_USIZE: usize = 16_000;
pub const SAMPLE_RATE_F32: f32 = 16_000.0;
pub const DOWNSAMPLE_RATE_F32: f32 = 320.0;
pub const LOGITS_PER_SECOND: f32 = SAMPLE_RATE_F32 / DOWNSAMPLE_RATE_F32;
// Zipa drops 2 logits on start and end
pub const EDGE_SKIPPED_LOGITS: usize = 2;

pub fn sample_count_to_duration(count: usize) -> Duration {
    Duration::from_secs_f32(count as f32 / SAMPLE_RATE_F32)
}

pub fn duration_to_sample_count(duration: &Duration) -> usize {
    (duration.as_secs_f32() * SAMPLE_RATE_F32).round() as usize
}

pub fn duration_to_logit_count(time: Duration) -> usize {
    (time.as_secs_f32() * LOGITS_PER_SECOND) as usize
}

pub fn logit_count_to_time(count: usize) -> Duration {
    Duration::from_secs_f32(count as f32 / LOGITS_PER_SECOND)
}

pub fn unix_timestamp_now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}
