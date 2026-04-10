use egui::{PopupCloseBehavior, RichText, Ui};
use tracing::Level;

use crate::ui::color::{DEBUG_COLOR, ERROR_COLOR, INFO_COLOR, TRACE_COLOR, WARN_COLOR};
use crate::ui::labels::Labels;
use crate::ui::state::LevelFilter;

#[derive(Default)]
pub struct LevelMenuButton<'a> {
    state: Option<&'a mut LevelFilter>,
    max_level: Option<Level>,
    labels: Option<&'a Labels>,
}

impl<'a> LevelMenuButton<'a> {
    pub fn state(mut self, v: &'a mut LevelFilter) -> Self {
        self.state = Some(v);
        self
    }

    pub fn max_level(mut self, level: Level) -> Self {
        self.max_level = Some(level);
        self
    }

    pub fn labels(mut self, labels: &'a Labels) -> Self {
        self.labels = Some(labels);
        self
    }

    pub fn show(mut self, ui: &mut Ui) {
        let state = self.state.as_mut().unwrap();
        let max = self.max_level.unwrap_or(Level::TRACE);
        let default_labels = Labels::default();
        let labels = self.labels.unwrap_or(&default_labels);
        let button = ui.button(labels.level.as_ref());

        egui::Popup::menu(&button)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| {
                ui.label(labels.level_filter.as_ref());
                if max >= Level::TRACE {
                    ui.add(egui::Checkbox::new(
                        &mut state.trace,
                        RichText::new("TRACE").color(TRACE_COLOR),
                    ));
                }
                if max >= Level::DEBUG {
                    ui.add(egui::Checkbox::new(
                        &mut state.debug,
                        RichText::new("DEBUG").color(DEBUG_COLOR),
                    ));
                }
                if max >= Level::INFO {
                    ui.add(egui::Checkbox::new(
                        &mut state.info,
                        RichText::new("INFO").color(INFO_COLOR),
                    ));
                }
                if max >= Level::WARN {
                    ui.add(egui::Checkbox::new(
                        &mut state.warn,
                        RichText::new("WARN").color(WARN_COLOR),
                    ));
                }
                ui.add(egui::Checkbox::new(
                    &mut state.error,
                    RichText::new("ERROR").color(ERROR_COLOR),
                ));
            });
    }
}
