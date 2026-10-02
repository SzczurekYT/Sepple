use std::{f64::consts::TAU, ops::Range, time::Duration};

use super::timeline_data::{
    AudioSegment, BufferSnapshot, SessionConfig, TimelineData, TokenEvent, VadScoreEvent, WordEvent,
};

const SAMPLE_RATE: usize = 16_000;
const SESSION: usize = 12 * SAMPLE_RATE;
const CHUNK: usize = 512;
const PRE_ROLL: usize = 10 * CHUNK;
const WINDOW: usize = SAMPLE_RATE;
const ADVANCE: usize = 11_200;
const CUT: usize = 2_400;
const WINDOW_LEAD: usize = 3_040;
const TOKEN_LEAD: usize = 800;
const ENVELOPE_EDGE: usize = 960;
const RAMP: f64 = 0.16;
const FLOOR: f64 = 0.05;
const RAMP_TOP: f64 = 0.8;
const PEAK_LOW: f64 = 0.6;
const PEAK_HIGH: f64 = 0.95;
const SYLLABLE_HZ: f64 = 4.0;

const UTTERANCES: [(usize, usize); 3] = [(12_800, 54_400), (73_600, 126_400), (145_600, 185_600)];
const PLAN: [&[(&str, &[&str], f64)]; 3] = [
    &[("ˈunvaksɒm", &["ˈun", "va", "ks", "ɒm"], 0.15)],
    &[
        ("vəˈluɡoː", &["və", "ˈlu", "ɡoː"], 0.28),
        ("plɒka", &["pl", "ɒk", "a"], 0.15),
    ],
    &[("lirɔ", &["l", "i", "r", "ɔ"], 0.22)],
];
const SPACING: [f64; 4] = [0.035, 0.041, 0.047, 0.05];

impl TimelineData {
    pub fn mockup() -> Self {
        let mut timeline = Self::default();
        let samples = raw_samples();

        let mut scores = score_events();
        apply_gate(&mut scores, &timeline.config);
        let runs = gate_runs(&scores);
        let windows = utterance_windows(&runs);
        let (tokens, snapshots, words) = build_events(&windows);

        timeline.post_vad = post_vad_segments(&runs, &samples);
        timeline.vad_scores = scores;
        timeline.chunks = windows
            .iter()
            .flatten()
            .map(|&start| AudioSegment {
                start: at(start),
                samples: samples[start..start + WINDOW].to_vec(),
            })
            .collect();
        timeline.tokens = tokens;
        timeline.snapshots = snapshots;
        timeline.words = words;
        timeline.raw = AudioSegment {
            start: Duration::ZERO,
            samples,
        };
        timeline
    }
}

fn at(sample: usize) -> Duration {
    Duration::from_secs_f64(sample as f64 / SAMPLE_RATE as f64)
}

fn raw_samples() -> Vec<f32> {
    (0..SESSION)
        .map(|i| {
            let t = i as f64 / SAMPLE_RATE as f64;
            let voiced = 0.55 * (TAU * 196.0 * t).sin()
                + 0.30 * (TAU * 588.0 * t).sin()
                + 0.15 * (TAU * 1176.0 * t).sin();
            let hum = 0.01 * (TAU * 63.0 * t).sin();
            (envelope(i) * voiced + hum) as f32
        })
        .collect()
}

fn envelope(i: usize) -> f64 {
    for &(start, end) in &UTTERANCES {
        if (start..end).contains(&i) {
            let attack = (i - start) as f64 / ENVELOPE_EDGE as f64;
            let release = (end - i) as f64 / ENVELOPE_EDGE as f64;
            let t = (i - start) as f64 / SAMPLE_RATE as f64;
            let syllabic = 0.55 + 0.45 * (TAU * SYLLABLE_HZ * t).sin();
            return 0.8 * attack.min(1.0) * release.min(1.0) * syllabic;
        }
    }
    0.0
}

fn score_events() -> Vec<VadScoreEvent> {
    (0..SESSION / CHUNK)
        .map(|i| {
            let middle = (i as f64 + 0.5) * CHUNK as f64 / SAMPLE_RATE as f64;
            VadScoreEvent {
                span: at(i * CHUNK)..at((i + 1) * CHUNK),
                score: score_at(middle) as f32,
                gate_open: false,
            }
        })
        .collect()
}

fn score_at(middle: f64) -> f64 {
    for &(start, end) in &UTTERANCES {
        let start = start as f64 / SAMPLE_RATE as f64;
        let end = end as f64 / SAMPLE_RATE as f64;
        if (start - RAMP..start).contains(&middle) {
            let rise = (middle - (start - RAMP)) / RAMP;
            return FLOOR + rise * (RAMP_TOP - FLOOR);
        }
        if (start..end).contains(&middle) {
            let syllable = 0.5 + 0.5 * (TAU * 3.0 * (middle - start)).sin();
            return PEAK_LOW + (PEAK_HIGH - PEAK_LOW) * syllable;
        }
        if (end..end + RAMP).contains(&middle) {
            let fall = (middle - end) / RAMP;
            return RAMP_TOP - fall * (RAMP_TOP - FLOOR);
        }
    }
    FLOOR
}

fn apply_gate(scores: &mut [VadScoreEvent], config: &SessionConfig) {
    let mut open = false;
    let mut rising = 0;
    let mut falling = 0;
    let mut prev = FLOOR as f32;
    for event in scores.iter_mut() {
        let score = event.score;
        if open {
            if score < config.vad_end_threshold && score <= prev {
                falling += 1;
            } else {
                falling = 0;
            }
            if falling >= 2 {
                open = false;
                rising = 0;
            }
        } else {
            if score >= config.vad_start_threshold && score >= prev {
                rising += 1;
            } else {
                rising = 0;
            }
            if rising >= 2 {
                open = true;
                falling = 0;
            }
        }
        event.gate_open = open;
        prev = score;
    }
}

fn gate_runs(scores: &[VadScoreEvent]) -> Vec<Range<usize>> {
    let mut runs = Vec::new();
    let mut i = 0;
    while i < scores.len() {
        if scores[i].gate_open {
            let start = i;
            while i < scores.len() && scores[i].gate_open {
                i += 1;
            }
            runs.push(start..i);
        } else {
            i += 1;
        }
    }
    runs
}

fn post_vad_segments(runs: &[Range<usize>], samples: &[f32]) -> Vec<AudioSegment> {
    runs.iter()
        .map(|run| {
            let from = run.start * CHUNK;
            let to = run.end * CHUNK;
            let from = from.saturating_sub(PRE_ROLL);
            AudioSegment {
                start: at(from),
                samples: samples[from..to].to_vec(),
            }
        })
        .collect()
}

fn utterance_windows(runs: &[Range<usize>]) -> Vec<Vec<usize>> {
    runs.iter()
        .map(|run| {
            let mut starts = Vec::new();
            let mut start = run.start * CHUNK - WINDOW_LEAD;
            while start < run.end * CHUNK && start + WINDOW <= SESSION {
                starts.push(start);
                start += ADVANCE;
            }
            starts
        })
        .collect()
}

fn build_events(windows: &[Vec<usize>]) -> (Vec<TokenEvent>, Vec<BufferSnapshot>, Vec<WordEvent>) {
    let mut tokens = Vec::new();
    let mut snapshots = Vec::new();
    let mut words = Vec::new();
    let mut buffer = String::new();

    for (u, starts) in windows.iter().enumerate() {
        let plan = PLAN[u];
        let mut batches: Vec<(usize, &str)> = Vec::new();
        for (index, (_, word_batches, _)) in plan.iter().enumerate() {
            for &batch in *word_batches {
                batches.push((index, batch));
            }
        }

        let utt_start = UTTERANCES[u].0;
        let mut firsts: Vec<Option<usize>> = vec![None; plan.len()];

        for (window_index, &window_start) in starts.iter().enumerate() {
            let Some(&(word_index, batch)) = batches.get(window_index) else {
                continue;
            };
            let mut time = (window_start + CUT + TOKEN_LEAD).max(utt_start + TOKEN_LEAD);
            for (i, symbol) in batch.chars().enumerate() {
                if i > 0 {
                    time += (SPACING[tokens.len() % SPACING.len()] * SAMPLE_RATE as f64) as usize;
                }
                if firsts[word_index].is_none() {
                    firsts[word_index] = Some(time);
                }
                tokens.push(TokenEvent {
                    time: at(time),
                    symbol: symbol.to_string(),
                });
            }

            buffer.push_str(batch);
            snapshots.push(BufferSnapshot {
                at: at(time),
                text: buffer.clone(),
            });

            let last_of_word = batches
                .get(window_index + 1)
                .is_none_or(|&(next, _)| next != word_index);
            if last_of_word {
                let (word, _, latency) = plan[word_index];
                let first = firsts[word_index].expect("word batch is never empty");
                let detected_at = time + (latency * SAMPLE_RATE as f64) as usize;
                words.push(WordEvent {
                    word: word.to_string(),
                    span: at(first)..at(time),
                    detected_at: at(detected_at),
                });
                buffer.clear();
                snapshots.push(BufferSnapshot {
                    at: at(detected_at),
                    text: String::new(),
                });
            }
        }

        if u + 1 < UTTERANCES.len() {
            let gap = (UTTERANCES[u].1 + UTTERANCES[u + 1].0) / 2;
            buffer.clear();
            snapshots.push(BufferSnapshot {
                at: at(gap),
                text: String::new(),
            });
        }
    }

    (tokens, snapshots, words)
}
