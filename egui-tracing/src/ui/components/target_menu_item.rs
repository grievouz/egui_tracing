use egui::Ui;
use globset::Glob;

use crate::string::Ellipse;

pub struct TargetMenuItem<'a, T> {
    target: &'a Glob,
    delete_label: &'a str,
    on_clicked: Option<T>,
}

impl<'a, T> TargetMenuItem<'a, T> {
    pub fn new(target: &'a Glob, delete_label: &'a str) -> Self {
        Self {
            target,
            delete_label,
            on_clicked: None,
        }
    }
}

impl<'a, T> TargetMenuItem<'a, T>
where
    T: FnMut(),
{
    pub fn on_clicked(mut self, v: T) -> Self {
        self.on_clicked = Some(v);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        ui.separator();
        let pattern = self.target.glob().to_owned();
        ui.horizontal(|ui| {
            ui.label(pattern.truncate_graphemes(18))
                .on_hover_text(&pattern);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(self.delete_label).clicked() {
                    self.on_clicked.unwrap()();
                }
            });
        });
    }
}
