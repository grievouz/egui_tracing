use egui::{vec2, Response, Sense, Ui};

pub static PADDING_LEFT: f32 = 4.0;

pub fn table_header(
    ui: &mut Ui,
    min_width: Option<f32>,
    content: impl FnOnce(&mut Ui),
) -> Response {
    ui.horizontal(|ui| {
        if let Some(min_width) = min_width {
            ui.set_width(min_width);
        }
        let available_space = ui.available_size_before_wrap();
        let size = vec2(PADDING_LEFT, available_space.y);
        let (rect, response) = ui.allocate_at_least(size, Sense::hover());
        if ui.is_rect_visible(response.rect) {
            let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
            let painter = ui.painter();
            painter.vline(rect.left(), rect.top()..=rect.bottom(), stroke);
        }

        content(ui);
    })
    .response
}
