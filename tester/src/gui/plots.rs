use eframe::egui::{self, Align, Grid, Layout, Ui, Vec2};
use egui_plot::{Plot, PlotBounds};

use crate::gui::time_axis;

use super::App;

const PLOT_PADDING: f32 = 8.0;
const WEIGHTS: [f32; 8] = [4.0, 4.0, 2.0, 4.0, 1.0, 1.0, 1.0, 2.0];
const LABELS: [&str; 8] = [
    "Waveform",
    "VAD Filtered",
    "Silero VAD",
    "Chunks",
    "Tokens",
    "Buffer",
    "Words",
    "Latency",
];

fn y_bounds(index: usize) -> (f64, f64) {
    match index {
        0 | 1 | 3 => (-1.0, 1.0),
        _ => (0.0, 1.0),
    }
}

pub fn render(_app: &App, ui: &mut Ui) {
    let available = (ui.available_height() - time_axis::HEIGHT - 2.0 * PLOT_PADDING).max(0.0);
    let weight_sum: f32 = WEIGHTS.iter().sum();

    let plot_cursor_link = ui.make_persistent_id("plot_strips");

    ui.allocate_space(Vec2::new(0.0, PLOT_PADDING));

    Grid::new("plot_grid")
        .num_columns(2)
        .spacing(Vec2::ZERO)
        .show(ui, |ui| {
            for (i, &weight) in WEIGHTS.iter().enumerate() {
                let height = weight / weight_sum * available;
                ui.horizontal(|ui| {
                    ui.label(LABELS[i]);
                    ui.allocate_space(Vec2::new(PLOT_PADDING, 0.0));
                });
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.allocate_space(Vec2::new(PLOT_PADDING, 0.0));
                    Plot::new(format!("strip{}", i + 1))
                        .height(height)
                        .auto_bounds(egui::Vec2b::from(false))
                        .show_axes([false, false])
                        .show_grid([false, false])
                        .allow_boxed_zoom(false)
                        .link_cursor(plot_cursor_link, [true, false])
                        .show(ui, |plot_ui| {
                            let (y_min, y_max) = y_bounds(i);
                            plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                                [0.0, y_min],
                                [10.0, y_max],
                            ));
                        });
                });
                ui.end_row();
            }

            ui.allocate_space(Vec2::new(1.0, 1.0));
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.allocate_space(Vec2::new(PLOT_PADDING, 0.0));
                time_axis::render(ui, plot_cursor_link);
            });
            ui.end_row();
        });
}
