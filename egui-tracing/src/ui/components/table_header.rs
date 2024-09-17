use egui::{vec2, Response, Sense, Ui};

use super::common::{set_common_props, CommonProps};
use super::ChildFn;

pub static PADDING_LEFT: f32 = 4.0;

pub struct TableHeader<T> {
    common_props: Option<CommonProps>,
    children: Option<T>,
}

impl<T> Default for TableHeader<T> {
    fn default() -> Self {
        Self {
            common_props: None,
            children: None,
        }
    }
}

impl<T> TableHeader<T>
where
    T: ChildFn,
{
    pub fn common_props(mut self, v: CommonProps) -> Self {
        self.common_props = Some(v);
        self
    }

    pub fn children(mut self, v: T) -> Self {
        self.children = Some(v);
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

            (self.children.unwrap())(ui)
        })
        .response
    }
}
