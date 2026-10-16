//! Kaldi-style log-mel filterbank, matching the feature extraction Zipa was
//! trained with (lhotse's `Fbank`, which is a port of torchaudio's kaldi
//! compliance module).
//!
//! Reference: `lhotse/features/kaldi/layers.py`. The defaults used by Zipa are
//! `FbankConfig(num_filters=80, dither=0.0, snip_edges=False)`, everything else
//! left at the dataclass defaults (`remove_dc_offset=True`, `preemph_coeff=0.97`,
//! `window_type="povey"`, `use_energy=False`, `use_fft_mag=False`,
//! `low_freq=20.0`, `high_freq=-400.0`, `norm_filters=False`).
//!
//! The time-domain stages (framing, DC removal, pre-emphasis, windowing) run in
//! plain `f32`; the FFT, power spectrum, mel projection and log run as burn
//! tensors so they can live on the same device as the model.

use burn::{
    Tensor,
    signal::{hann_window, rfft},
    tensor::{Device, TensorData},
};

pub const SAMPLE_RATE: usize = 16_000;
pub const FRAME_LENGTH: usize = 400;
pub const FRAME_SHIFT: usize = 160;
pub const FFT_LENGTH: usize = 512;
pub const NUM_MEL_BINS: usize = 80;

const PREEMPHASIS_COEFF: f32 = 0.97;
const LOW_FREQ: f64 = 20.0;
const NYQUIST: f64 = SAMPLE_RATE as f64 / 2.0;
/// lhotse's `high_freq = -400.0` is relative to Nyquist: 8000 - 400 = 7600 Hz.
const HIGH_FREQ: f64 = NYQUIST - 400.0;
const MEL_SCALE: f64 = 1127.0;
const MEL_BREAK_FREQ: f64 = 700.0;
/// Kaldi clamps the filterbank energy at `std::numeric_limits<float>::epsilon`,
/// i.e. `torch.finfo(torch.float).eps`.
const LOG_FLOOR: f32 = f32::EPSILON;

/// Number of frames `_get_strided_batch` produces for `snip_edges=False`.
pub fn num_frames(num_samples: usize) -> usize {
    (num_samples + FRAME_SHIFT / 2) / FRAME_SHIFT
}

fn lin_to_mel(freq: f64) -> f64 {
    MEL_SCALE * (1.0 + freq / MEL_BREAK_FREQ).ln()
}

/// `[FFT_LENGTH / 2 + 1, NUM_MEL_BINS]`, row-major, ready for a matmul against
/// the power spectrum. This is `get_mel_banks(...)` padded to a zero Nyquist
/// column and transposed.
fn mel_filterbank() -> Vec<f32> {
    let num_fft_bins = FFT_LENGTH / 2;
    let fft_bin_width = SAMPLE_RATE as f64 / FFT_LENGTH as f64;
    let mel_low = lin_to_mel(LOW_FREQ);
    let mel_high = lin_to_mel(HIGH_FREQ);
    let mel_delta = (mel_high - mel_low) / (NUM_MEL_BINS as f64 + 1.0);

    let mut matrix = vec![0.0f32; (FFT_LENGTH / 2 + 1) * NUM_MEL_BINS];
    for bin in 0..NUM_MEL_BINS {
        let left = mel_low + bin as f64 * mel_delta;
        let center = mel_low + (bin as f64 + 1.0) * mel_delta;
        let right = mel_low + (bin as f64 + 2.0) * mel_delta;
        for fft_bin in 0..num_fft_bins {
            let mel = lin_to_mel(fft_bin_width * fft_bin as f64);
            let up_slope = (mel - left) / (center - left);
            let down_slope = (right - mel) / (right - center);
            let value = up_slope.min(down_slope).max(0.0);
            matrix[fft_bin * NUM_MEL_BINS + bin] = value as f32;
        }
    }
    matrix
}

pub struct Fbank {
    device: Device,
    window: Vec<f32>,
    mel: Tensor<2>,
}

impl Fbank {
    pub fn new(device: Device) -> Self {
        let window = hann_window(FRAME_LENGTH, false, &device)
            .powf_scalar(0.85)
            .into_data()
            .try_into_vec_as::<f32>()
            .expect("fbank window to be f32");

        let mel = Tensor::from_data(
            TensorData::new(mel_filterbank(), [FFT_LENGTH / 2 + 1, NUM_MEL_BINS]),
            &device,
        );

        Self {
            device,
            window,
            mel,
        }
    }

    /// Reflect-pads the waveform, cuts it into `num_frames` overlapping
    /// `FRAME_LENGTH` windows, then applies DC removal, pre-emphasis and the
    /// Povey window, returning them zero-padded to `FFT_LENGTH`.
    fn frame(&self, samples: &[f32]) -> (Vec<f32>, usize) {
        let num_frames = num_frames(samples.len());
        assert!(
            num_frames > 0 && samples.len() >= FRAME_LENGTH,
            "fbank needs at least {FRAME_LENGTH} samples, got {}",
            samples.len()
        );

        let npad_left = (FRAME_LENGTH - FRAME_SHIFT) / 2;
        let padded_len = (num_frames - 1) * FRAME_SHIFT + FRAME_LENGTH;
        let npad_right = padded_len - samples.len() - npad_left;

        let mut padded = Vec::with_capacity(padded_len);
        padded.extend(samples[..npad_left].iter().rev());
        padded.extend_from_slice(samples);
        padded.extend(samples[samples.len() - npad_right..].iter().rev());

        let mut frames = vec![0.0f32; num_frames * FFT_LENGTH];
        for frame_index in 0..num_frames {
            let start = frame_index * FRAME_SHIFT;
            let frame = &padded[start..start + FRAME_LENGTH];
            let out =
                &mut frames[frame_index * FFT_LENGTH..frame_index * FFT_LENGTH + FRAME_LENGTH];

            let mean = frame.iter().sum::<f32>() / FRAME_LENGTH as f32;
            let mut previous = frame[0] - mean;
            for (index, &sample) in frame.iter().enumerate() {
                let centered = sample - mean;
                out[index] = (centered - PREEMPHASIS_COEFF * previous) * self.window[index];
                previous = centered;
            }
        }
        (frames, num_frames)
    }

    /// `[num_frames, NUM_MEL_BINS]` log-mel features for a whole waveform.
    pub fn compute(&self, samples: &[f32]) -> Tensor<2> {
        let (frames, num_frames) = self.frame(samples);

        let frames = Tensor::<2>::from_data(
            TensorData::new(frames, [num_frames, FFT_LENGTH]),
            &self.device,
        );
        let (real, imag) = rfft(frames, 1, Some(FFT_LENGTH));

        // `remove_dc_offset` and the window/emphasis are folded into `frames` above.
        let power = real.clone().mul(real).add(imag.clone().mul(imag));
        let energy = power.matmul(self.mel.clone());

        energy.clamp_min(LOG_FLOOR).log()
    }

    /// Convenience for tests and debugging: the raw feature matrix as
    /// `[num_frames, NUM_MEL_BINS]` row-major `f32`.
    pub fn compute_vec(&self, samples: &[f32]) -> Vec<f32> {
        self.compute(samples)
            .into_data()
            .try_into_vec_as::<f32>()
            .expect("fbank output to be f32")
    }

    pub fn window(&self) -> &[f32] {
        &self.window
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn frame_count_matches_lhotse() {
        assert_eq!(num_frames(16_000), 100);
        assert_eq!(num_frames(16_001), 100);
        assert_eq!(num_frames(16_040), 100);
        assert_eq!(num_frames(16_079), 100);
        assert_eq!(num_frames(16_080), 101);
        assert_eq!(num_frames(1_171 * FRAME_SHIFT + 100), 1172);
    }

    #[test]
    fn mel_filterbank_layout() {
        let matrix = mel_filterbank();
        assert_eq!(matrix.len(), (FFT_LENGTH / 2 + 1) * NUM_MEL_BINS);

        for bin in 0..NUM_MEL_BINS {
            let sum: f32 = (0..FFT_LENGTH / 2 + 1)
                .map(|i| matrix[i * NUM_MEL_BINS + bin])
                .sum();
            assert!(sum > 0.0, "mel bin {bin} has no energy");
            assert!(
                matrix[NUM_MEL_BINS * (FFT_LENGTH / 2) + bin] == 0.0,
                "Nyquist row should be zero"
            );
        }

        // DC sits below the 20 Hz band edge, so the first row is all zero.
        assert!((0..NUM_MEL_BINS).all(|bin| matrix[bin] == 0.0));
    }

    #[test]
    fn povey_window_is_hann_pow_085() {
        let fbank = Fbank::new(Device::default());
        let window = fbank.window();
        assert_eq!(window.len(), FRAME_LENGTH);

        for (n, value) in window.iter().enumerate() {
            let angle = 2.0 * PI * n as f64 / (FRAME_LENGTH - 1) as f64;
            let expected = (0.5 - 0.5 * angle.cos()).powf(0.85) as f32;
            assert!(
                (value - expected).abs() < 1e-5,
                "window[{n}] = {value}, expected {expected}"
            );
        }

        assert!((window[0] - 0.0).abs() < 1e-6);
        assert!((window[FRAME_LENGTH - 1] - 0.0).abs() < 1e-6);
        assert!((window[FRAME_LENGTH / 2] - 1.0).abs() < 1e-3);
    }

    #[test]
    fn silences_map_to_the_log_floor() {
        let fbank = Fbank::new(Device::default());
        let features = fbank.compute_vec(&vec![0.0f32; 16_000]);
        assert_eq!(features.len(), num_frames(16_000) * NUM_MEL_BINS);
        let expected = LOG_FLOOR.ln();
        for value in features {
            assert!(
                (value - expected).abs() < 1e-4,
                "silent frame should clamp to ln(eps) = {expected}, got {value}"
            );
        }
    }
}
