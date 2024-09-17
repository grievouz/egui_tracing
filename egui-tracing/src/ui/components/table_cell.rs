use egui::{Response, Ui};

use super::common::{set_common_props, CommonProps};
use super::table_header;
use super::ChildFn;

#[derive(Debug)]
pub struct TableCell<T>
where
    T: ChildFn,
{
    common_props: Option<CommonProps>,
    children: Option<T>,
}

impl<F> Default for TableCell<F>
where
    F: ChildFn,
{
    fn default() -> Self {
        Self {
            common_props: None,
            children: None,
        }
    }
}

impl<T> TableCell<T>
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
            ui.add_space(table_header::PADDING_LEFT * 2.0 + 10.0);

            (self.children.unwrap())(ui);
        })
        .response
    }
}
