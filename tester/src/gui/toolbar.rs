use eframe::egui::{self, Align, ComboBox, Layout};

use super::App;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExportSource {
    Raw,
    PostVad,
    SwChunks,
}

impl ExportSource {
    pub const ALL: [ExportSource; 3] = [
        ExportSource::Raw,
        ExportSource::PostVad,
        ExportSource::SwChunks,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ExportSource::Raw => "Raw",
            ExportSource::PostVad => "Post-VAD",
            ExportSource::SwChunks => "SW chunks",
        }
    }

    pub fn preview(self) -> &'static str {
        match self {
            ExportSource::Raw => "selected range",
            ExportSource::PostVad | ExportSource::SwChunks => "N chunks",
        }
    }
}

pub fn render(app: &mut App, ui: &mut egui::Ui) {
    ui.style_mut().spacing.item_spacing.x = 8.0;
    ui.style_mut().spacing.button_padding = egui::vec2(12.0, 8.0);
    ui.horizontal_centered(|ui| {
        if ui.button("Start").clicked() {
            app.start();
        }
        if ui.button("Stop").clicked() {
            app.stop();
        }
        ui.label(if app.running { "running" } else { "stopped" });
        ui.label(format!("dropped events: {}", app.dropped));

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ComboBox::from_id_salt("export_source")
                .selected_text(app.export_source.name())
                .show_ui(ui, |ui| {
                    for source in ExportSource::ALL {
                        ui.selectable_value(&mut app.export_source, source, source.name());
                    }
                });

            ui.label(app.export_source.preview().to_owned() + " from");
            ui.add_enabled(false, egui::Button::new("Export"));
        });
    });
}
