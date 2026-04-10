use egui::{PopupCloseBehavior, Ui};
use globset::Glob;

use super::target_menu_item::TargetMenuItem;
use crate::ui::labels::Labels;
use crate::ui::state::TargetFilter;

pub struct TargetMenuButton<'a> {
    state: Option<&'a mut TargetFilter>,
    labels: &'a Labels,
}

impl<'a> TargetMenuButton<'a> {
    pub fn new(labels: &'a Labels) -> Self {
        Self {
            state: None,
            labels,
        }
    }

    pub fn state(mut self, v: &'a mut TargetFilter) -> Self {
        self.state = Some(v);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let state = self.state.unwrap();
        let button = ui.button(self.labels.target.as_ref());

        egui::Popup::menu(&button)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.label(self.labels.target_filter.as_ref());

                let (input, add_button) = ui
                    .horizontal(|ui| {
                        let input = ui.add(
                            egui::TextEdit::singleline(&mut state.input)
                                .hint_text(self.labels.target_placeholder.as_ref()),
                        );
                        let add = ui.button(self.labels.add.as_ref());
                        (input, add)
                    })
                    .inner;

                if add_button.clicked()
                    || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    state.targets.push(Glob::new(&state.input).unwrap());
                    state.input = "".to_owned();
                }

                let delete_label = self.labels.delete.as_ref();
                for (i, target) in state.targets.clone().iter().enumerate() {
                    TargetMenuItem::new(target, delete_label)
                        .on_clicked(|| {
                            state.targets.remove(i);
                        })
                        .show(ui);
                }
            });
    }
}
