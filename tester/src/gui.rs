use std::path::PathBuf;

use eframe::{
    Frame,
    egui::{self, Panel, Ui},
};

mod mockup;
mod plots;
mod time_axis;
mod timeline_data;
mod toolbar;
mod vad;
mod waveform;

use sepple::units::sample_count_to_duration;
use timeline_data::TimelineData;
use toolbar::ExportSource;

pub const LOOKAHEAD_VIEW_SPAN_FRACTION: f64 = 0.25;
pub const MAX_SPAN: f64 = 60.0;

pub fn run(file: Option<PathBuf>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("sepple tester")
            .with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "sepple tester",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(App {
                file,
                timeline: TimelineData::mockup(),
                running: false,
                dropped: 0,
                export_source: ExportSource::Raw,
                view: ViewState {
                    start: 0.0,
                    end: 10.0,
                    target_span: 10.0,
                },
            }))
        }),
    )
}

pub struct App {
    #[allow(dead_code)]
    file: Option<PathBuf>,
    timeline: TimelineData,
    running: bool,
    dropped: usize,
    export_source: ExportSource,
    view: ViewState,
}

impl App {
    fn start(&mut self) {
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn last_sample_x(&self) -> f64 {
        let raw = &self.timeline.raw;
        (raw.start + sample_count_to_duration(raw.samples.len())).as_secs_f64()
    }

    fn view_max_x(&self) -> f64 {
        self.last_sample_x() + LOOKAHEAD_VIEW_SPAN_FRACTION * self.view.target_span
    }

    pub fn set_view_span(&mut self, new_span: f64) {
        self.set_target_span(new_span);
        let view_max_x = self.view_max_x();
        self.view.set_span(new_span, view_max_x);
    }

    pub fn set_view_bounds(&mut self, new_start: f64, new_end: f64) {
        self.set_target_span(new_end - new_start);
        let view_max_x = self.view_max_x();
        self.view.set_bounds(new_start, new_end, view_max_x);
    }

    pub fn set_target_span(&mut self, target_span: f64) {
        self.view.target_span = target_span
            .min(sample_count_to_duration(self.timeline.raw.samples.len()).as_secs_f64())
            .min(MAX_SPAN)
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        Panel::top("toolbar").show(ui, |ui| {
            toolbar::render(self, ui);
        });

        if self.running {
            self.view.end = self.last_sample_x();
        }

        plots::render(self, ui);
    }
}

pub struct ViewState {
    pub start: f64,
    pub end: f64,
    target_span: f64,
}

impl ViewState {
    pub fn span(&self) -> f64 {
        self.end - self.start
    }

    fn set_span(&mut self, new_span: f64, view_max_x: f64) {
        let old_span = self.span();
        let edge_diff = (new_span - old_span) / 2.0;
        self.set_bounds(self.start - edge_diff, self.end + edge_diff, view_max_x);
    }

    fn set_bounds(&mut self, new_start: f64, new_end: f64, view_max_x: f64) {
        (self.start, self.end) = self.clamp_bounds(new_start, new_end, view_max_x);
    }

    fn clamp_bounds(&self, mut new_start: f64, mut new_end: f64, view_max_x: f64) -> (f64, f64) {
        if new_end > view_max_x {
            new_start -= new_end - view_max_x;
            new_start = new_start.max(0.0);
            new_end = view_max_x;
        }

        if new_start < 0.0 {
            new_end -= new_start;
            new_end = new_end.min(view_max_x);
            new_start = 0.0;
        }

        (new_start, new_end)
    }
}
