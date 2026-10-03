use std::path::PathBuf;

use eframe::egui::{self, Panel};

mod mockup;
mod plots;
mod time_axis;
mod timeline_data;
mod toolbar;

use timeline_data::TimelineData;
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
            }))
        }),
    )
}

pub struct App {
    #[allow(dead_code)]
    file: Option<PathBuf>,
    #[allow(dead_code)]
    timeline: TimelineData,
    running: bool,
    dropped: usize,
    export_source: ExportSource,
}

impl App {
    fn start(&mut self) {
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Panel::top("toolbar").show(ui, |ui| {
            toolbar::render(self, ui);
        });
        plots::render(self, ui);
    }
}
