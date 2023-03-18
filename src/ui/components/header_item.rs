use egui::{Ui, Widget};

pub struct HeaderItem<T>
where
    T: FnOnce(&mut Ui) -> (),
{
    min_width: Option<f32>,
    child: Option<T>,
}

impl<T> HeaderItem<T>
where
    T: FnOnce(&mut Ui) -> (),
{
    pub fn set_child(mut self, child: T) -> Self {
        self.child = Some(child);
        self
    }

    pub fn set_min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }
}

impl<T> Default for HeaderItem<T>
where
    T: FnOnce(&mut Ui) -> (),
{
    fn default() -> Self {
        Self {
            min_width: None,
            child: None,
        }
    }
}

impl<T> Widget for HeaderItem<T>
where
    T: FnOnce(&mut Ui) -> (),
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if let Some(min_width) = self.min_width {
                ui.set_min_width(min_width);
            }
            ui.separator();
            ui.add_space(2.0);
            if let Some(child) = self.child {
                child(ui);
            }
        })
        .response
    }
}
