use std::iter;

use eframe::egui::{self, Align, Color32, Grid, Id, Layout, Stroke, Ui, Vec2, Vec2b};
use egui_plot::{Plot, PlotUi};

use crate::gui::{time_axis, waveform};

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
const PLOT_FNS: [fn(&App, &mut PlotUi<'_>); 8] = [
    render_raw_waveform,
    render_noop,
    render_noop,
    render_noop,
    render_noop,
    render_noop,
    render_noop,
    render_noop,
];

fn y_bounds(index: usize) -> (f64, f64) {
    match index {
        0 | 1 | 3 => (-1.0, 1.0),
        _ => (0.0, 1.0),
    }
}

fn base_plot(id: &str, running: bool, height: f32, cursor_link: Id, axis_link: Id) -> Plot<'_> {
    Plot::new(id)
        .height(height)
        .link_axis(axis_link, Vec2b::new(true, false))
        .auto_bounds(Vec2b::from(false))
        .show_axes([false, false])
        .show_grid([false, false])
        .allow_boxed_zoom(false)
        .allow_drag(Vec2b::new(!running, false))
        .allow_scroll(Vec2b::new(!running, false))
        .allow_zoom(Vec2b::new(true, false))
        .pan_pointer_button(egui::PointerButton::Secondary)
        .link_cursor(cursor_link, [true, false])
}

pub fn render(app: &mut App, ui: &mut Ui) {
    let available = (ui.available_height() - time_axis::HEIGHT - 2.0 * PLOT_PADDING).max(0.0);
    let weight_sum: f32 = WEIGHTS.iter().sum();

    let plot_cursor_link = ui.make_persistent_id("plot_strips");
    let time_link = ui.make_persistent_id("time_link");

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
                    base_plot(
                        &format!("strip{}", i + 1),
                        app.running,
                        height,
                        plot_cursor_link,
                        time_link,
                    )
                    .show(ui, |plot_ui| {
                        if i == 0 {
                            plot_ui.set_plot_bounds_x(app.view.start..=app.view.end);
                        }
                        let (y_min, y_max) = y_bounds(i);
                        plot_ui.set_plot_bounds_y(y_min..=y_max);

                        (PLOT_FNS[i])(app, plot_ui);
                    });
                });
                ui.end_row();
            }

            ui.allocate_space(Vec2::new(1.0, 1.0));
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.allocate_space(Vec2::new(PLOT_PADDING, 0.0));
                time_axis::render(ui, app, plot_cursor_link, time_link)
            });
            ui.end_row();
        });
}

pub fn render_noop(_app: &App, _plot_ui: &mut PlotUi) {}

const RAW_FILL_COLOR: Color32 = Color32::from_rgba_premultiplied(100, 160, 255, 70);
const RAW_STROKE_COLOR: Color32 = Color32::from_rgb(100, 160, 255);

pub fn render_raw_waveform(app: &App, plot_ui: &mut PlotUi) {
    waveform::draw(
        app,
        plot_ui,
        iter::once((app.timeline.raw.start, app.timeline.raw.samples.as_slice())),
        "raw",
        0.0,
        1.0,
        RAW_FILL_COLOR,
        Stroke::new(1.0, RAW_STROKE_COLOR),
    );
}
