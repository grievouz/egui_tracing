use egui::{Response, Ui};

use super::table_header;

pub fn table_cell(ui: &mut Ui, min_width: f32, content: impl FnOnce(&mut Ui)) -> Response {
    ui.horizontal(|ui| {
        ui.set_width(min_width);
        ui.add_space(table_header::PADDING_LEFT * 2.0 + 10.0);

        content(ui);
    })
    .response
}
