use eframe::egui::{Color32, RichText};
use egui_plot::{PlotPoint, PlotUi, Points, Text};

use super::App;

const TOKEN_COLOR: Color32 = Color32::from_rgb(200, 130, 255);
const LABEL_MIN_GAP_PX: f64 = 10.0;

pub fn render_tokens(app: &App, plot_ui: &mut PlotUi) {
    let x_min = app.view.start;
    let x_max = app.view.end;
    let span = (x_max - x_min).max(1e-6);
    let width_px = plot_ui.response().rect.width();
    let px_per_sec = width_px as f64 / span;

    let times: Vec<f64> = app
        .timeline
        .tokens
        .iter()
        .map(|token| token.time.as_secs_f64())
        .filter(|&t| t >= x_min && t <= x_max)
        .collect();

    let min_gap_px = times
        .windows(2)
        .map(|w| (w[1] - w[0]) * px_per_sec)
        .fold(f64::INFINITY, f64::min);
    if min_gap_px >= LABEL_MIN_GAP_PX {
        for (i, token) in app.timeline.tokens.iter().enumerate() {
            let t = token.time.as_secs_f64();
            if t < x_min || t > x_max {
                continue;
            }
            plot_ui.text(
                Text::new(
                    format!("strip5_lbl_{i}"),
                    PlotPoint::new(t, 0.5),
                    RichText::new(&token.symbol).size(20.0),
                )
                .color(TOKEN_COLOR),
            );
        }
    } else {
        plot_ui.points(
            Points::new(
                "strip5_pts",
                times.iter().map(|&t| [t, 0.5]).collect::<Vec<[f64; 2]>>(),
            )
            .radius(2.0)
            .color(TOKEN_COLOR),
        );
    }
}
