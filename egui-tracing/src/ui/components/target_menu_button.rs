use egui::{PopupCloseBehavior, Ui};
use globset::Glob;

use super::target_menu_item::TargetMenuItem;
use crate::ui::labels::Labels;
use crate::ui::state::TargetFilter;

#[derive(Default)]
pub struct TargetMenuButton<'a> {
    state: Option<&'a mut TargetFilter>,
    labels: Option<&'a Labels>,
}

impl<'a> TargetMenuButton<'a> {
    pub fn state(mut self, v: &'a mut TargetFilter) -> Self {
        self.state = Some(v);
        self
    }

    pub fn labels(mut self, labels: &'a Labels) -> Self {
        self.labels = Some(labels);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let state = self.state.unwrap();
        let default_labels = Labels::default();
        let labels = self.labels.unwrap_or(&default_labels);
        let button = ui.button(labels.target.as_ref());

        egui::Popup::menu(&button)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.label(labels.target_filter.as_ref());

                let (input, add_button) = ui
                    .horizontal(|ui| {
                        let input = ui
                            .text_edit_singleline(&mut state.input)
                            .on_hover_text(labels.target_placeholder.as_ref());
                        let add = ui.button(labels.add.as_ref());
                        (input, add)
                    })
                    .inner;

                if add_button.clicked()
                    || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    state.targets.push(Glob::new(&state.input).unwrap());
                    state.input = "".to_owned();
                }

                let delete_label = &labels.delete;
                for (i, target) in state.targets.clone().iter().enumerate() {
                    TargetMenuItem::default()
                        .on_clicked(|| {
                            state.targets.remove(i);
                        })
                        .target(target)
                        .delete_label(delete_label)
                        .show(ui);
                }
            });
    }
}
