use egui::Ui;
use globset::Glob;

use crate::string::Ellipse;
use crate::ui::state::TargetFilter;

pub fn target_menu_button(ui: &mut Ui, state: &mut TargetFilter) {
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
            state.targets.insert(Glob::new(&state.input).unwrap());
            state.input = "".to_owned();
        }

        for target in state.targets.clone() {
            target_menu_item(ui, &target, || {
                state.targets.remove(&target);
            });
        }
    });
}

fn target_menu_item(ui: &mut Ui, target: &Glob, on_clicked: impl FnOnce()) {
    ui.separator();
    let pattern = target.glob().to_owned();
    ui.horizontal(|ui| {
        ui.label(pattern.truncate_graphemes(18))
            .on_hover_text(pattern);
        ui.add_space(ui.available_width() - 37.0);
        if ui.button("Delete").clicked() {
            on_clicked();
        }
    });
}
