use std::slice::Iter;

use egui::{Color32, Response, Ui};

use super::constants::SEPARATOR_SPACING;

pub fn table<T>(
    ui: &mut Ui,
    row_height: f32,
    values: Iter<&T>,
    on_clear: impl FnOnce(),
    header: impl FnOnce(&mut Ui),
    row: impl Fn(&mut Ui, &T),
) -> Response {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);

            ui.horizontal(|ui| {
                header(ui);
            });

            ui.add_space(ui.available_width() - 130.0);

            if ui
                .button("To Bottom")
                .on_hover_text("Scroll to Bottom")
                .clicked()
            {
                ui.scroll_to_rect(
                    egui::Rect {
                        min: egui::Pos2 { x: 0.0, y: 0.0 },
                        max: egui::Pos2 {
                            x: f32::MAX,
                            y: f32::MAX,
                        },
                    },
                    Some(egui::Align::Max),
                );
            }

            ui.separator();

            if ui.button("Clear").on_hover_text("Clear Events").clicked() {
                on_clear();
            }
        });

        ui.separator();

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show_rows(
                ui,
                row_height + SEPARATOR_SPACING,
                values.len(),
                |ui, range| {
                    for value in values.skip(range.start).take(range.len()) {
                        ui.horizontal(|ui| {
                            row(ui, value);
                        });
                        ui.separator();
                    }
                },
            )
    })
    .response
}
