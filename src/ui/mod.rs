mod color;
mod string;
mod time;

use egui::{Color32, Label, RichText};

use self::color::ToColor32;
use self::string::Ellipse;
use self::time::SpecificFormats;
use crate::tracing::EventCollector;

pub fn ui(collector: &EventCollector, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.set_height(26.0);
            ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
            ui.horizontal(|ui| {
                ui.set_min_width(80.0);
                ui.separator();
                ui.add_space(2.0);
                ui.label("Time");
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.set_min_width(40.0);
                ui.separator();
                ui.add_space(2.0);
                ui.menu_button("Level", |ui| {
                    ui.label("Level Message Filter");
                    ui.add(egui::Checkbox::new(
                        &mut true,
                        RichText::new("TRACE").color(tracing::Level::TRACE.to_color32()),
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut true,
                        RichText::new("DEBUG").color(tracing::Level::DEBUG.to_color32()),
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut true,
                        RichText::new("INFO").color(tracing::Level::INFO.to_color32()),
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut true,
                        RichText::new("WARN").color(tracing::Level::WARN.to_color32()),
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut true,
                        RichText::new("ERROR").color(tracing::Level::ERROR.to_color32()),
                    ));
                });
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.set_min_width(100.0);
                ui.separator();
                ui.add_space(2.0);
                ui.label("Target");
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.separator();
                ui.add_space(2.0);
                ui.label("Message");
            });
        });
        ui.separator();
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for event in collector.events() {
                    ui.horizontal(|ui| {
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.set_min_width(80.0);
                            ui.add_space(5.0);
                            ui.colored_label(Color32::GRAY, event.time.format_short())
                                .on_hover_text(event.time.format_detailed());
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.set_min_width(40.0);
                            ui.add_space(5.0);
                            ui.colored_label(event.level.to_color32(), event.level.as_str());
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.set_min_width(100.0);
                            ui.add_space(5.0);
                            ui.colored_label(Color32::GRAY, event.target.truncate_graphemes(14))
                                .on_hover_text(&event.target);
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            ui.add_space(5.0);
                            ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                            ui.add(Label::new(event.fields.get("message").unwrap()).wrap(true));
                        });
                    });
                    ui.separator();
                }
            });
    });
}
