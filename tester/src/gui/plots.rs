use std::iter;

use eframe::egui::{Align, Color32, Grid, Id, Layout, PointerButton, Ui, Vec2, Vec2b};
use egui_plot::{FilledArea, Plot, PlotResponse, PlotUi};

use crate::gui::tokens::{render_buffer_snapshots, render_tokens, render_words};
use crate::gui::vad::render_vad_state;
use crate::gui::{chunk::render_sw_chunks, delay::modify_latency_plot};
use crate::gui::{delay::render_latency, vad::modify_vad_plot};
use crate::gui::{time_axis, waveform};

use super::App;

type PlotRenderer = fn(&App, &mut PlotUi<'_>);
type PlotModifier = for<'a> fn(&'a App, Plot<'a>) -> Plot<'a>;

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
pub const CUT_BORDER_COLOR: Color32 = Color32::from_rgba_unmultiplied_const(255, 255, 255, 40);
pub const SELECTION_COLOR: Color32 = Color32::from_rgba_unmultiplied_const(255, 255, 255, 60);
const PLOT_FNS: [PlotRenderer; 8] = [
    render_raw_waveform,
    render_post_vad_waveform,
    render_vad_state,
    render_sw_chunks,
    render_tokens,
    render_buffer_snapshots,
    render_words,
    render_latency,
];
const PLOT_MODIFIERS: [Option<PlotModifier>; 8] = [
    None,
    None,
    Some(modify_vad_plot),
    None,
    None,
    None,
    None,
    Some(modify_latency_plot),
];

fn y_bounds(index: usize) -> (f64, f64) {
    match index {
        0 | 1 | 3 => (-1.0, 1.0),
        _ => (0.0, 1.0),
    }
}

fn base_plot<'a>(id: &str, running: bool, height: f32, cursor_link: Id, axis_link: Id) -> Plot<'a> {
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
        .pan_pointer_button(PointerButton::Secondary)
        .link_cursor(cursor_link, [true, false])
}

fn update_selection(app: &mut App, response: &PlotResponse<()>) {
    if app.running {
        return;
    }
    let Some(pointer) = response.response.interact_pointer_pos() else {
        return;
    };
    let x = response.transform.value_from_position(pointer).x;
    let response = &response.response;
    if response.drag_started_by(PointerButton::Primary) {
        app.selection = Some(x..x);
    } else if response.dragged_by(PointerButton::Primary) {
        if let Some(sel) = &mut app.selection {
            sel.end = x;
        }
    } else if response.drag_stopped() {
        if let Some(sel) = &app.selection {
            let (start, end) = (sel.start.min(sel.end), sel.start.max(sel.end));
            app.selection = Some(start..end);
        }
    } else if response.clicked() {
        app.selection = None;
    }
}

fn draw_selection_band(app: &App, plot_ui: &mut PlotUi) {
    let Some(sel) = &app.selection else {
        return;
    };
    if sel.end <= sel.start {
        return;
    }
    // Using f64::MIN/MAX does not work for some reason (nothing renders)
    // So we just use big numbers
    plot_ui.add(
        FilledArea::new(
            "selection_band",
            &[sel.start, sel.end],
            &[-1_000_000_000.0, -1_000_000_000.0],
            &[1_000_000_000.0, 1_000_000_000.0],
        )
        .fill_color(SELECTION_COLOR),
    );
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
                    let mut plot = base_plot(
                        &format!("strip{}", i + 1),
                        app.running,
                        height,
                        plot_cursor_link,
                        time_link,
                    );

                    if let Some(modifier) = PLOT_MODIFIERS[i] {
                        plot = (modifier)(app, plot);
                    };

                    let response = plot.show(ui, |plot_ui| {
                        if i == 0 {
                            plot_ui.set_plot_bounds_x(app.view.start..=app.view.end);
                        }
                        let (y_min, y_max) = y_bounds(i);
                        plot_ui.set_plot_bounds_y(y_min..=y_max);

                        (PLOT_FNS[i])(app, plot_ui);
                        draw_selection_band(app, plot_ui);
                    });
                    update_selection(app, &response);
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

pub fn render_raw_waveform(app: &App, plot_ui: &mut PlotUi) {
    waveform::draw(
        app,
        plot_ui,
        iter::once((app.timeline.raw.start, app.timeline.raw.samples.as_slice())),
        "raw",
        0.0,
        1.0,
    );
}

pub fn render_post_vad_waveform(app: &App, plot_ui: &mut PlotUi) {
    waveform::draw(
        app,
        plot_ui,
        app.timeline
            .post_vad
            .iter()
            .map(|segment| (segment.start, segment.samples.as_slice())),
        "post_vad",
        0.0,
        1.0,
    );
}
