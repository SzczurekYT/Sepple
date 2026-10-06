use eframe::egui::Color32;
use egui_plot::{FilledArea, HLine, HoverPosition, Line, Plot, PlotUi};

use crate::gui::timeline_data::VadScoreEvent;

use super::App;

const GATE_BAND_COLOR: Color32 = Color32::from_rgba_unmultiplied_const(80, 200, 120, 60);
const SCORE_LINE_COLOR: Color32 = Color32::from_rgb(255, 180, 60);
const THRESHOLD_LINE_COLOR: Color32 = Color32::from_rgb(255, 90, 90);

pub fn render_vad_state(app: &App, plot_ui: &mut PlotUi) {
    let scores = &app.timeline.vad_scores;
    if scores.is_empty() {
        return;
    }

    for (start, end) in gate_open_segments(scores) {
        let x0 = scores[start].span.start.as_secs_f64();
        let x1 = scores[end].span.end.as_secs_f64();
        plot_ui.add(
            FilledArea::new(
                format!("gate_band_{start}"),
                &[x0, x1],
                &[0.0, 0.0],
                &[1.0, 1.0],
            )
            .fill_color(GATE_BAND_COLOR),
        );
    }

    let points: Vec<[f64; 2]> = scores
        .iter()
        .flat_map(|event| {
            [
                [event.span.start.as_secs_f64(), event.score as f64],
                [event.span.end.as_secs_f64(), event.score as f64],
            ]
        })
        .collect();
    plot_ui.line(
        Line::new("vad_scores", points)
            .color(SCORE_LINE_COLOR)
            .allow_band(false),
    );

    plot_ui.hline(
        HLine::new(
            "vad_start_threshold",
            app.timeline.config.vad_start_threshold as f64,
        )
        .color(THRESHOLD_LINE_COLOR),
    );
    plot_ui.hline(
        HLine::new(
            "vad_end_threshold",
            app.timeline.config.vad_end_threshold as f64,
        )
        .color(THRESHOLD_LINE_COLOR),
    );
}

fn gate_open_segments(scores: &[VadScoreEvent]) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut run_start = None;
    for (i, event) in scores.iter().enumerate() {
        match (event.gate_open, run_start) {
            (true, None) => run_start = Some(i),
            (false, Some(start)) => {
                runs.push((start, i - 1));
                run_start = None;
            }
            _ => {}
        }
    }
    if let Some(start) = run_start {
        runs.push((start, scores.len() - 1));
    }
    runs
}

pub fn modify_vad_plot<'a>(app: &'a App, plot: Plot<'a>) -> Plot<'a> {
    plot.label_formatter(|hover| match hover {
        HoverPosition::NearDataPoint {
            plot_name,
            position,
            ..
        } if !plot_name.is_empty() => {
            for vad_event in &app.timeline.vad_scores {
                if (vad_event.span.start.as_secs_f64() - position.x).abs() < 0.1 {
                    return Some(format!("Score: {:.2}", vad_event.score));
                }
            }
            None
        }
        _ => None,
    })
    .show_grid([false, true])
}
