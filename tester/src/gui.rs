use std::path::PathBuf;

use eframe::egui;

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
        Box::new(move |_cc| Ok(Box::new(App { file }))),
    )
}

struct App {
    #[allow(dead_code)]
    file: Option<PathBuf>,
}

impl eframe::App for App {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}
}
