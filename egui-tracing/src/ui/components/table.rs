use std::slice::Iter;

use egui::{Response, Ui};

use super::constants::SEPARATOR_SPACING;

pub struct Table<'a, T> {
    row_height: Option<f32>,
    on_clear: Option<Box<dyn FnMut() + 'a>>,
    header: Option<Box<dyn FnMut(&mut Ui) + 'a>>,
    row: Option<Box<dyn FnMut(&mut Ui, &T) + 'a>>,
}

impl<'a, T> Default for Table<'a, T> {
    fn default() -> Self {
        Self {
            row_height: None,
            on_clear: None,
            header: None,
            row: None,
        }
    }
}

impl<'a, T> Table<'a, T> {
    pub fn row_height(mut self, v: f32) -> Self {
        self.row_height = Some(v);
        self
    }

    pub fn on_clear(mut self, v: impl FnMut() + 'a) -> Self {
        self.on_clear = Some(Box::new(v));
        self
    }

    pub fn header(mut self, v: impl FnMut(&mut Ui) + 'a) -> Self {
        self.header = Some(Box::new(v));
        self
    }

    pub fn row(mut self, v: impl FnMut(&mut Ui, &T) + 'a) -> Self {
        self.row = Some(Box::new(v));
        self
    }

    pub fn show(self, ui: &mut Ui, values: Iter<&T>) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                //ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);

                ui.horizontal(|ui| {
                    (self.header.unwrap())(ui);
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
                    (self.on_clear.unwrap())();
                }
            });

            ui.separator();

            let mut row = self.row.unwrap();
            egui::ScrollArea::vertical()
                .auto_shrink([true, false])
                .stick_to_bottom(true)
                .show_rows(
                    ui,
                    self.row_height.unwrap() + SEPARATOR_SPACING,
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
}
