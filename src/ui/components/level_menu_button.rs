use egui::{RichText, Ui};

use super::common::{set_common_props, CommonProps};
use crate::ui::color::{DEBUG_COLOR, ERROR_COLOR, INFO_COLOR, TRACE_COLOR, WARN_COLOR};
use crate::ui::state::LevelFilter;

#[derive(Default)]
pub struct LevelMenuButton<'a> {
    state: Option<&'a mut LevelFilter>,
    common_props: Option<CommonProps>,
}

impl<'a> LevelMenuButton<'a> {
    pub fn state(mut self, v: &'a mut LevelFilter) -> Self {
        self.state = Some(v);
        self
    }


    pub fn show(mut self, ui: &mut Ui) {
        let state = self.state.as_mut().unwrap();
        ui.menu_button("Level", |ui| {
            set_common_props(ui, &self.common_props);
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
}
