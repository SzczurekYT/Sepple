use eframe::egui::{Align, Button, ComboBox, Layout, Slider, Ui, Vec2};
use rfd::FileDialog;

use super::App;
use super::export;

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
}

pub fn render(app: &mut App, ui: &mut Ui) {
    ui.style_mut().spacing.item_spacing.x = 8.0;
    ui.style_mut().spacing.button_padding = Vec2::new(12.0, 8.0);
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

            let preview = match app.export_source {
                ExportSource::Raw => "selected range".to_owned(),
                ExportSource::PostVad | ExportSource::SwChunks => {
                    format!("{} chunks", export::export_count(app).unwrap_or(0))
                }
            };
            ui.label(format!("{preview} from"));

            let can_export = !app.running && app.selection.is_some();
            if ui.add_enabled(can_export, Button::new("Export")).clicked() {
                let path = FileDialog::new()
                    .set_file_name("exported.wav")
                    .add_filter("Wav", &["wav"])
                    .save_file();
                if let Some(path) = path {
                    export::export(app, &path);
                }
            }

            ui.allocate_space(Vec2::new(25.0, 0.0));

            let mut span = app.view.span();
            let span_max = app.view_max_x();
            let slider_response = ui.add(Slider::new(&mut span, 1.0..=span_max));
            if slider_response.changed() {
                app.set_view_span(span);
            }

            ui.label("zoom");
        });
    });
}
