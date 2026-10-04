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

use sepple::units::sample_count_to_duration;
use timeline_data::{AudioSegment, TimelineData};
use toolbar::ExportSource;

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
                    center: 5.0,
                    span: 10.0,
                },
            }))
        }),
    )
}

pub struct ViewState {
    pub center: f64,
    pub span: f64,
}

pub struct App {
    #[allow(dead_code)]
    file: Option<PathBuf>,
    #[allow(dead_code)]
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
}

fn live_edge(raw: &AudioSegment) -> f64 {
    (raw.start + sample_count_to_duration(raw.samples.len())).as_secs_f64()
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        Panel::top("toolbar").show(ui, |ui| {
            toolbar::render(self, ui);
        });

        if self.running {
            let edge = live_edge(&self.timeline.raw);
            self.view.center = edge - self.view.span / 2.0;
        }

        plots::render(self, ui);
    }
}
