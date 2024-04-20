use egui::Ui;
use globset::Glob;

use super::target_menu_item::TargetMenuItem;
use crate::ui::state::TargetFilter;

#[derive(Default)]
pub struct TargetMenuButton<'a> {
    state: Option<&'a mut TargetFilter>,
}

impl<'a> TargetMenuButton<'a> {
    pub fn state(mut self, v: &'a mut TargetFilter) -> Self {
        self.state = Some(v);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let state = self.state.unwrap();
        ui.menu_button("Target", |ui| {
            ui.label("Target Filter");

            let (input, add_button) = ui
                .horizontal(|ui| {
                    let input = ui
                        .text_edit_singleline(&mut state.input)
                        .on_hover_text("example: eframe::*");
                    let button = ui.button("Add");
                    (input, button)
                })
                .inner;

            if add_button.clicked()
                || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                state.targets.push(Glob::new(&state.input).unwrap());
                state.input = "".to_owned();
            }

            for (i, target) in state.targets.clone().iter().enumerate() {
                TargetMenuItem::default()
                    .on_clicked(|| {
                        state.targets.remove(i);
                    })
                    .target(&target)
                    .show(ui);
            }
        });
    }
}
