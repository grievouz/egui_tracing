use std::marker::PhantomData;
use std::slice::Iter;

use egui::{Response, Ui};

use super::constants::SEPARATOR_SPACING;
use super::ChildFn;

pub struct Table<OnClearFn, HeaderFn, RowFn, Item> {
    row_height: Option<f32>,
    on_clear: Option<OnClearFn>,
    header: Option<HeaderFn>,
    row: Option<RowFn>,
    _marker: PhantomData<Item>,
}

impl<OnClearFn, HeaderFn, RowFn, Item> Default for Table<OnClearFn, HeaderFn, RowFn, Item> {
    fn default() -> Self {
        Self {
            row_height: None,
            on_clear: None,
            header: None,
            row: None,
            _marker: PhantomData,
        }
    }
}

impl<OnClearFn, HeaderFn, RowFn, Item> Table<OnClearFn, HeaderFn, RowFn, Item>
where
    OnClearFn: FnMut(),
    HeaderFn: ChildFn,
    RowFn: FnMut(&mut Ui, &Item),
{
    pub fn row_height(mut self, v: f32) -> Self {
        self.row_height = Some(v);
        self
    }

    pub fn on_clear(mut self, v: OnClearFn) -> Self {
        self.on_clear = Some(v);
        self
    }

    pub fn header(mut self, v: HeaderFn) -> Self {
        self.header = Some(v);
        self
    }

    pub fn row(mut self, v: RowFn) -> Self {
        self.row = Some(v);
        self
    }

    pub fn show(self, ui: &mut Ui, values: Iter<&Item>) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
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
