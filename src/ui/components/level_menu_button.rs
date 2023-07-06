use egui::{RichText, Ui};

use crate::ui::color::{DEBUG_COLOR, ERROR_COLOR, INFO_COLOR, TRACE_COLOR, WARN_COLOR};
use crate::ui::state::LevelFilter;

pub fn level_menu_button(ui: &mut Ui, state: &mut LevelFilter) {
    ui.menu_button("Level", |ui| {
        ui.label("Level Filter");
        ui.add(egui::Checkbox::new(
            &mut state.trace,
            RichText::new("TRACE").color(TRACE_COLOR),
        ));
        ui.add(egui::Checkbox::new(
            &mut state.debug,
            RichText::new("DEBUG").color(DEBUG_COLOR),
        ));
        ui.add(egui::Checkbox::new(
            &mut state.info,
            RichText::new("INFO").color(INFO_COLOR),
        ));
        ui.add(egui::Checkbox::new(
            &mut state.warn,
            RichText::new("WARN").color(WARN_COLOR),
        ));
        ui.add(egui::Checkbox::new(
            &mut state.error,
            RichText::new("ERROR").color(ERROR_COLOR),
        ));
    });
}
