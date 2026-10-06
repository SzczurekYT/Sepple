use std::time::Duration;

use eframe::egui::{Color32, Stroke};
use egui_plot::{FilledArea, PlotUi};
use sepple::units::SAMPLE_RATE_F32;

use super::App;

const SAMPLE_RATE: f64 = SAMPLE_RATE_F32 as f64;
const WAVEFORM_FILL_COLOR: Color32 = Color32::from_rgba_premultiplied(100, 160, 255, 70);
const WAVEFORM_STROKE_COLOR: Color32 = Color32::from_rgb(100, 160, 255);

#[derive(Default)]
pub struct Envelope {
    pub xs: Vec<f64>,
    pub ys_min: Vec<f64>,
    pub ys_max: Vec<f64>,
}

/// Sample index range `[i_start, i_end)` covering `[x_min, x_max)`, or `None`
/// when nothing is visible.
fn visible_range(
    start: Duration,
    samples: &[f32],
    x_min: f64,
    x_max: f64,
) -> Option<(usize, usize)> {
    if x_max <= x_min || samples.is_empty() {
        return None;
    }
    let start_s = start.as_secs_f64();
    let i_start = ((x_min - start_s) * SAMPLE_RATE).ceil().max(0.0) as usize;
    let i_end = ((x_max - start_s) * SAMPLE_RATE)
        .ceil()
        .min(samples.len() as f64) as usize;
    (i_start < i_end).then_some((i_start, i_end))
}

/// Per-pixel-column `(min, max)` of the visible samples.
fn column_extrema(
    visible: &[f32],
    start_s: f64,
    i_start: usize,
    x_min: f64,
    dx: f64,
    n_buckets: usize,
) -> (Vec<f32>, Vec<f32>) {
    let mut mins = vec![f32::INFINITY; n_buckets];
    let mut maxs = vec![f32::NEG_INFINITY; n_buckets];
    for (j, &sample) in visible.iter().enumerate() {
        let t = start_s + (i_start + j) as f64 / SAMPLE_RATE;
        let column = ((t - x_min) / dx) as usize;
        if column >= n_buckets {
            break;
        }
        mins[column] = mins[column].min(sample);
        maxs[column] = maxs[column].max(sample);
    }
    (mins, maxs)
}

fn build_envelope(mins: &[f32], maxs: &[f32], x_min: f64, dx: f64) -> Envelope {
    let mut env = Envelope::default();
    for (column, (&min, &max)) in mins.iter().zip(maxs).enumerate() {
        if min.is_finite() {
            env.xs.push(x_min + (column as f64 + 0.5) * dx);
            env.ys_min.push(min as f64);
            env.ys_max.push(max as f64);
        }
    }
    env
}

pub fn envelope(
    start: Duration,
    samples: &[f32],
    x_min: f64,
    x_max: f64,
    width_px: f32,
) -> Envelope {
    if width_px <= 0.0 {
        return Envelope::default();
    }
    let Some((i_start, i_end)) = visible_range(start, samples, x_min, x_max) else {
        return Envelope::default();
    };
    let n_buckets = (width_px as usize).max(1);
    let dx = (x_max - x_min) / n_buckets as f64;
    let (mins, maxs) = column_extrema(
        &samples[i_start..i_end],
        start.as_secs_f64(),
        i_start,
        x_min,
        dx,
        n_buckets,
    );
    build_envelope(&mins, &maxs, x_min, dx)
}

#[allow(clippy::too_many_arguments)]
pub fn draw<'a>(
    app: &App,
    plot_ui: &mut PlotUi,
    segments: impl Iterator<Item = (Duration, &'a [f32])>,
    name_prefix: &str,
    y_offset: f64,
    y_scale: f64,
) {
    let x_min = app.view.start;
    let x_max = app.view.end;
    let width_px = plot_ui.response().rect.width();
    for (i, (start, samples)) in segments.enumerate() {
        let mut env = envelope(start, samples, x_min, x_max, width_px);
        if env.xs.is_empty() {
            continue;
        }
        for y in env.ys_min.iter_mut().chain(env.ys_max.iter_mut()) {
            *y = y_offset + y_scale * *y;
        }
        plot_ui.add(
            FilledArea::new(
                format!("{name_prefix}_{i}"),
                &env.xs,
                &env.ys_min,
                &env.ys_max,
            )
            .fill_color(WAVEFORM_FILL_COLOR)
            .stroke(Stroke::new(1.0, WAVEFORM_STROKE_COLOR)),
        );
    }
}
