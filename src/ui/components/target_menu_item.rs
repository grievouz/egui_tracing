use egui::Ui;
use globset::Glob;

use crate::string::Ellipse;

#[derive(Default)]
pub struct TargetMenuItem<'a> {
    target: Option<&'a Glob>,
    on_clicked: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a> TargetMenuItem<'a> {
    pub fn target(mut self, v: &'a Glob) -> Self {
        self.target = Some(v);
        self
    }

    pub fn on_clicked(mut self, v: impl FnMut() + 'a) -> Self {
        self.on_clicked = Some(Box::new(v));
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
