use std::borrow::Cow;

use eframe::egui::{Color32, RichText};
use egui_plot::{FilledArea, PlotPoint, PlotUi, Points, Text};

use super::App;

const TOKEN_COLOR: Color32 = Color32::from_rgb(200, 130, 255);
const BUFFER_COLOR: Color32 = Color32::from_rgb(80, 180, 255);
const WORD_COLOR: Color32 = Color32::from_rgb(255, 140, 80);
const LABEL_MIN_GAP_PX: f64 = 10.0;
const LABEL_TEXT_SIZE: f32 = 20.0;
const SNAPSHOT_MAX_CHARS: usize = 20;

fn draw_label(
    plot_ui: &mut PlotUi,
    name: impl Into<String>,
    x: f64,
    y: f64,
    text: &str,
    color: Color32,
) {
    plot_ui.text(
        Text::new(
            name,
            PlotPoint::new(x, y),
            RichText::new(text).size(LABEL_TEXT_SIZE),
        )
        .color(color),
    );
}

fn truncate_tail(text: &str) -> Cow<'_, str> {
    let char_count = text.chars().count();
    if char_count <= SNAPSHOT_MAX_CHARS {
        return Cow::Borrowed(text);
    }
    let skip = char_count - SNAPSHOT_MAX_CHARS;
    let byte_idx = text.char_indices().nth(skip).map(|(i, _)| i).unwrap_or(0);
    Cow::Owned(format!("{}...", &text[byte_idx..]))
}

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
            draw_label(
                plot_ui,
                format!("strip5_lbl_{i}"),
                t,
                0.5,
                &token.symbol,
                TOKEN_COLOR,
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

pub fn render_buffer_snapshots(app: &App, plot_ui: &mut PlotUi) {
    for (i, snapshot) in app.timeline.snapshots.iter().enumerate() {
        let x = snapshot.at.as_secs_f64();
        if x < app.view.start || x > app.view.end {
            continue;
        }
        draw_label(
            plot_ui,
            format!("strip6_lbl_{i}"),
            x,
            0.5,
            &truncate_tail(&snapshot.text),
            BUFFER_COLOR,
        );
    }
}

pub fn render_words(app: &App, plot_ui: &mut PlotUi) {
    for (i, word) in app.timeline.words.iter().enumerate() {
        let start = word.span.start.as_secs_f64();
        let end = word.span.end.as_secs_f64();
        if end < app.view.start || start > app.view.end {
            continue;
        }
        plot_ui.add(
            FilledArea::new(
                format!("word_bar_{i}"),
                &[start, end],
                &[0.3, 0.3],
                &[0.5, 0.5],
            )
            .fill_color(WORD_COLOR),
        );
        draw_label(
            plot_ui,
            format!("word_lbl_{i}"),
            (start + end) / 2.0,
            0.7,
            &word.word,
            WORD_COLOR,
        );
    }
}
