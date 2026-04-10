use egui::Ui;
use globset::Glob;

use crate::string::Ellipse;

pub struct TargetMenuItem<'a, T> {
    target: Option<&'a Glob>,
    on_clicked: Option<T>,
    delete_label: Option<&'a str>,
}

impl<'a, T> Default for TargetMenuItem<'a, T> {
    fn default() -> Self {
        Self {
            target: None,
            on_clicked: None,
            delete_label: None,
        }
    }
}

impl<'a, T> TargetMenuItem<'a, T>
where
    T: FnMut(),
{
    pub fn target(mut self, v: &'a Glob) -> Self {
        self.target = Some(v);
        self
    }

    pub fn on_clicked(mut self, v: T) -> Self {
        self.on_clicked = Some(v);
        self
    }

    pub fn delete_label(mut self, label: &'a str) -> Self {
        self.delete_label = Some(label);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        ui.separator();
        let pattern = self.target.unwrap().glob().to_owned();
        ui.horizontal(|ui| {
            ui.label(pattern.truncate_graphemes(18))
                .on_hover_text(&pattern);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(self.delete_label.unwrap_or("Delete")).clicked() {
                    self.on_clicked.unwrap()();
                }
            });
        });
    }
}
