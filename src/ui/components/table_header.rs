use egui::{Ui, Widget};

pub struct TableHeader<'a> {
    pub min_width: Option<f32>,
    child: Option<Box<dyn FnOnce(&mut Ui) + 'a>>,
}

impl<'a> TableHeader<'a> {
    pub fn set_child(mut self, child: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn set_min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }
}

impl<'a> Default for TableHeader<'a> {
    fn default() -> Self {
        Self {
            min_width: None,
            child: None,
        }
    }
}

impl<'a> Widget for TableHeader<'a> {
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
