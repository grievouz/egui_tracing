use egui::Ui;
use globset::Glob;

use crate::string::Ellipse;

pub struct TargetMenuItem<'a, T> {
    target: Option<&'a Glob>,
    on_clicked: Option<T>,
}

impl<'a, T> Default for TargetMenuItem<'a, T> {
    fn default() -> Self {
        Self {
            target: None,
            on_clicked: None,
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

    pub fn show(self, ui: &mut Ui) {
        ui.separator();
        let pattern = self.target.unwrap().glob().to_owned();
        ui.horizontal(|ui| {
            ui.label(pattern.truncate_graphemes(18))
                .on_hover_text(pattern);
            ui.add_space(ui.available_width() - 43.0);
            if ui.button("Delete").clicked() {
                self.on_clicked.unwrap()();
            }
        });
    }
}
