use egui::{RichText, Widget};

use super::{
    color::{ToColor32, DEBUG_COLOR, ERROR_COLOR, INFO_COLOR, TRACE_COLOR, WARN_COLOR},
    LevelFilter,
};

pub struct LevelFilterMenuButton<'a> {
    state: &'a mut LevelFilter,
}

impl<'a> LevelFilterMenuButton<'a> {
    pub(super) fn new(state: &'a mut LevelFilter) -> Self {
        Self {
            state
        }
    }
}

impl<'a> Widget for LevelFilterMenuButton<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.menu_button("Level", |ui| {
            ui.label("Level Message Filter");
            ui.add(egui::Checkbox::new(
                &mut self.state.trace,
                RichText::new("TRACE").color(TRACE_COLOR),
            ));
            ui.add(egui::Checkbox::new(
                &mut self.state.debug,
                RichText::new("DEBUG").color(DEBUG_COLOR),
            ));
            ui.add(egui::Checkbox::new(
                &mut self.state.info,
                RichText::new("INFO").color(INFO_COLOR),
            ));
            ui.add(egui::Checkbox::new(
                &mut self.state.warn,
                RichText::new("WARN").color(WARN_COLOR),
            ));
            ui.add(egui::Checkbox::new(
                &mut self.state.error,
                RichText::new("ERROR").color(ERROR_COLOR),
            ));
        }).response
    }
}
