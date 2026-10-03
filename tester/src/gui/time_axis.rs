use std::ops::RangeInclusive;

use eframe::egui::{self, Color32, Id, Ui};
use egui_plot::{GridInput, GridMark, Plot, PlotBounds};

pub const HEIGHT: f32 = 30.0;
const TICK_STEPS: [f64; 10] = [0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 15.0, 30.0, 60.0];

pub fn render(ui: &mut Ui, cursor_link_id: Id) {
    ui.horizontal(|ui| {
        ui.visuals_mut().extreme_bg_color = Color32::from_rgb(27, 27, 27);
        Plot::new("time_axis")
            .height(30.0)
            .auto_bounds(egui::Vec2b::from(false))
            .show_axes([true, false])
            .show_grid([true, false])
            .link_cursor(cursor_link_id, [true, false])
            .allow_drag([false, false])
            .allow_zoom(false)
            .allow_boxed_zoom(false)
            .allow_axis_zoom_drag(false)
            .x_grid_spacer(time_grid_spacer)
            .x_axis_formatter(format_tick)
            .show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 0.0], [10.0, 1.0]));
            });
    });
}

fn tick_step(range: f64) -> f64 {
    TICK_STEPS
        .iter()
        .find(|&&step| range / step <= 7.0)
        .copied()
        .unwrap_or(60.0)
}

fn time_grid_spacer(input: GridInput) -> Vec<GridMark> {
    let (min, max) = input.bounds;
    let step = tick_step(max - min);
    let first = (min / step).ceil() as i64;
    let last = (max / step).floor() as i64;
    (first..=last)
        .map(|i| GridMark {
            value: i as f64 * step,
            step_size: step,
        })
        .collect()
}

fn format_tick(mark: GridMark, _range: &RangeInclusive<f64>) -> String {
    if mark.value.fract() == 0.0 {
        format!("{}", mark.value as i64)
    } else {
        format!("{:.1}", mark.value)
    }
}
