use std::ops::Range;

use egui::{Color32, Response, Ui, Widget};

use super::table_header::TableHeader;

#[derive(Default)]
pub struct Table<'a> {
    header_height: Option<f32>,
    headers: Vec<TableHeader<'a>>,
    rows: Option<Box<dyn FnOnce(&mut Ui, Range<usize>) + 'a>>,
    rows_count: Option<usize>,
    row_height: Option<f32>,
}

impl<'a> Table<'a> {
    pub fn set_header_height(mut self, header_height: f32) -> Self {
        self.header_height = Some(header_height);
        self
    }

    pub fn add_header(mut self, header: TableHeader<'a>) -> Self {
        self.header_height = header.min_width;
        self.headers.push(header);
        self
    }

    pub fn show_rows(
        mut self,
        row_height: f32,
        count: usize,
        rows: impl FnOnce(&mut Ui, Range<usize>) + 'a,
    ) -> Self {
        self.row_height = Some(row_height);
        self.rows_count = Some(count);
        self.rows = Some(Box::new(rows));
        self
    }
}

impl<'a> Widget for Table<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // ui.set_height(self.header_height.unwrap());
                ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);

                for header in self.headers {
                    ui.add(header);
                    ui.add_space(5.0); // TODO: dont add space at the end
                }

                // ui.add_space(ui.available_width() - 130.0);

                // elements.jump_bottom_btn =
                //     Some(ui.button("To Bottom").on_hover_text("Scroll to Bottom"));

                // ui.separator();

                // if ui.button("Clear").on_hover_text("Clear Events").clicked() {
                //     self.collector.clear();
                // }
            });

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show_rows(
                    ui,
                    self.row_height.unwrap(),
                    self.rows_count.unwrap(),
                    self.rows.unwrap(),
                )
        })
        .response
    }
}
