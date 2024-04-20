use egui::{Response, Ui};

use super::common::{set_common_props, CommonProps};
use super::table_header;

#[derive(Default)]
pub struct TableCell<'a> {
    common_props: Option<CommonProps>,
    children: Option<Box<dyn FnMut(&mut Ui) + 'a>>,
}

impl<'a> TableCell<'a> {
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
            ui.add_space(table_header::PADDING_LEFT * 2.0 + 10.0);

            (self.children.unwrap().as_mut())(ui);
        })
        .response
    }
}
