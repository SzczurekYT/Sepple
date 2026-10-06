use std::iter;

use eframe::egui::Color32;
use egui_plot::{FilledArea, PlotUi};
use sepple::units::sample_count_to_duration;

use crate::gui::plots::CUT_BORDER_COLOR;
use crate::gui::waveform;

use super::App;

pub fn render_sw_chunks(app: &App, plot_ui: &mut PlotUi) {
    let config = &app.timeline.config;
    let window_size = config.window_size.as_secs_f64();
    let advance = config.advance().as_secs_f64();
    let lane_count = (window_size / advance).ceil().max(1.0) as usize;
    let cut_left = config.cut_left.as_secs_f64();
    let cut_right = config.cut_right.as_secs_f64();
    let x_min = app.view.start;
    let x_max = app.view.end;
    let y_scale = 1.0 / lane_count as f64;

    for (n, chunk) in app.timeline.chunks.iter().enumerate() {
        let chunk_start = chunk.start.as_secs_f64();
        let chunk_end = chunk_start + sample_count_to_duration(chunk.samples.len()).as_secs_f64();
        if chunk_end < x_min || chunk_start > x_max {
            continue;
        }
        let lane = n % lane_count;
        let y_offset = 1.0 - (2 * lane + 1) as f64 / lane_count as f64;
        let y_low = y_offset - y_scale;
        let y_high = y_offset + y_scale;

        let left_end = (chunk_start + cut_left).clamp(x_min, x_max);
        draw_shaded_range(
            plot_ui,
            chunk_start.clamp(x_min, x_max),
            left_end,
            y_low,
            y_high,
            CUT_BORDER_COLOR,
            format!("cut_left_{n}"),
        );
        let right_start = (chunk_end - cut_right).clamp(x_min, x_max);
        draw_shaded_range(
            plot_ui,
            right_start,
            chunk_end.clamp(x_min, x_max),
            y_low,
            y_high,
            CUT_BORDER_COLOR,
            format!("cut_right_{n}"),
        );

        waveform::draw(
            app,
            plot_ui,
            iter::once((chunk.start, chunk.samples.as_slice())),
            "chunk",
            y_offset,
            y_scale,
        );
    }
}

fn draw_shaded_range(
    plot_ui: &mut PlotUi,
    x0: f64,
    x1: f64,
    y_lo: f64,
    y_hi: f64,
    color: Color32,
    name: impl Into<String>,
) {
    if x1 <= x0 {
        return;
    }
    plot_ui.add(FilledArea::new(name, &[x0, x1], &[y_lo, y_lo], &[y_hi, y_hi]).fill_color(color));
}
