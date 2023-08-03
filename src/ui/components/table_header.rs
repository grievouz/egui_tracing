use egui::{vec2, Response, Sense, Ui};

use super::common::{set_common_props, CommonProps};

pub static PADDING_LEFT: f32 = 4.0;

#[derive(Default)]
pub struct TableHeader<'a> {
    common_props: Option<CommonProps>,
    children: Option<Box<dyn FnMut(&mut Ui) + 'a>>,
}

impl<'a> TableHeader<'a> {
    pub fn common_props(mut self, v: CommonProps) -> Self {
        self.common_props = Some(v);
        self
    }

    pub fn children(mut self, v: impl FnMut(&mut Ui) + 'a) -> Self {
        self.children = Some(Box::new(v));
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            set_common_props(ui, &self.common_props);
            let available_space = ui.available_size_before_wrap();
            let size = vec2(PADDING_LEFT, available_space.y);
            let (rect, response) = ui.allocate_at_least(size, Sense::hover());
            if ui.is_rect_visible(response.rect) {
                let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
                let painter = ui.painter();
                painter.vline(rect.left(), rect.top()..=rect.bottom(), stroke);
            }

            (self.children.unwrap().as_mut())(ui)
        })
        .response
    }
}
