use eframe::egui::Color32;
use egui_plot::{Line, PlotUi, Points};

use super::App;

const LATENCY_COLOR: Color32 = Color32::from_rgb(255, 100, 180);

pub fn render_latency(app: &App, plot_ui: &mut PlotUi) {
    let x_min = app.view.start;
    let x_max = app.view.end;

    let points: Vec<(f64, f64)> = app
        .timeline
        .words
        .iter()
        .filter(|word| {
            let x = word.detected_at.as_secs_f64();
            x >= x_min && x <= x_max
        })
        .map(|word| {
            (
                word.detected_at.as_secs_f64(),
                word.detected_at.saturating_sub(word.span.end).as_secs_f64(),
            )
        })
        .collect();

    if points.is_empty() {
        return;
    }

    let y_max = points
        .iter()
        .map(|(_, height)| *height)
        .fold(0.0, f64::max)
        .max(0.1)
        * 1.2;
    plot_ui.set_plot_bounds_y(0.0..=y_max);

    for (i, (x, height)) in points.iter().enumerate() {
        plot_ui.line(
            Line::new(format!("latency_stem_{i}"), vec![[*x, 0.0], [*x, *height]])
                .color(LATENCY_COLOR),
        );
        plot_ui.points(
            Points::new(format!("latency_pt_{i}"), vec![[*x, *height]])
                .radius(3.0)
                .color(LATENCY_COLOR),
        );
    }
}
