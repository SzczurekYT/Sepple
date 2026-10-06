use std::time::Duration;

use eframe::egui::{Color32, Stroke};
use egui_plot::{FilledArea, PlotUi};

use crate::gui::App;

const SAMPLE_RATE: f64 = 16_000.0;

pub struct Envelope {
    pub xs: Vec<f64>,
    pub ys_min: Vec<f64>,
    pub ys_max: Vec<f64>,
}

pub fn envelope(
    start: Duration,
    samples: &[f32],
    x_min: f64,
    x_max: f64,
    width_px: f32,
) -> Envelope {
    let mut env = Envelope {
        xs: Vec::new(),
        ys_min: Vec::new(),
        ys_max: Vec::new(),
    };
    if x_max <= x_min || width_px <= 0.0 || samples.is_empty() {
        return env;
    }
    let start_s = start.as_secs_f64();
    let i_start = ((x_min - start_s) * SAMPLE_RATE).ceil().max(0.0) as usize;
    let i_end = ((x_max - start_s) * SAMPLE_RATE)
        .ceil()
        .min(samples.len() as f64) as usize;
    if i_start >= i_end {
        return env;
    }
    let visible = &samples[i_start..i_end];
    let n_buckets = (width_px as usize).max(1);
    let dx = (x_max - x_min) / n_buckets as f64;
    let mut mins = vec![f32::INFINITY; n_buckets];
    let mut maxs = vec![f32::NEG_INFINITY; n_buckets];
    for (j, &sample) in visible.iter().enumerate() {
        let t = start_s + (i_start + j) as f64 / SAMPLE_RATE;
        let column = ((t - x_min) / dx) as usize;
        if column >= n_buckets {
            break;
        }
        if sample < mins[column] {
            mins[column] = sample;
        }
        if sample > maxs[column] {
            maxs[column] = sample;
        }
    }
    for column in 0..n_buckets {
        if mins[column].is_finite() {
            env.xs.push(x_min + (column as f64 + 0.5) * dx);
            env.ys_min.push(mins[column] as f64);
            env.ys_max.push(maxs[column] as f64);
        }
    }
    env
}

#[allow(clippy::too_many_arguments)]
pub fn draw<'a>(
    app: &App,
    plot_ui: &mut PlotUi,
    segments: impl Iterator<Item = (Duration, &'a [f32])>,
    name_prefix: &str,
    y_offset: f64,
    y_scale: f64,
    fill: Color32,
    stroke: Stroke,
) {
    let width_px = plot_ui.response().rect.width();
    for (i, (start, samples)) in segments.enumerate() {
        let mut env = envelope(start, samples, app.view.start, app.view.end, width_px);
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
            .fill_color(fill)
            .stroke(stroke),
        );
    }
}
