mod color;
mod components;
mod state;

use std::sync::{Arc, Mutex};

use egui::{Color32, Label, Response, TextStyle, Widget};
use globset::GlobSetBuilder;

use self::color::ToColor32;
use self::components::constants;
use self::components::level_menu_button::level_menu_button;
use self::components::table::table;
use self::components::table_cell::table_cell;
use self::components::table_header::table_header;
use self::components::target_menu_button::target_menu_button;
use self::state::LogsState;
use crate::string::Ellipse;
use crate::time::DateTimeFormatExt;
use crate::tracing::collector::EventCollector;

pub struct Logs {
    collector: EventCollector,
}

impl Logs {
    #[must_use]
    pub const fn new(collector: EventCollector) -> Self {
        Self { collector }
    }
}

impl Widget for Logs {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let id = ui.id();
        let state = ui.memory_mut(|mem| {
            mem.data
                .get_persisted_mut_or_insert_with(id, || Arc::new(Mutex::new(LogsState::default())))
                .clone()
        });
        let mut state = state.lock().unwrap();
        let events = self.collector.events();

        let glob = {
            let mut glob = GlobSetBuilder::new();
            for target in state.target_filter.targets.clone() {
                glob.add(target);
            }
            glob.build().unwrap()
        };

        let filtered_events = events
            .iter()
            .filter(|event| state.level_filter.get(event.level) && !glob.is_match(&event.target))
            .collect::<Vec<_>>();

        let row_height = constants::SEPARATOR_SPACING
            + ui.style().text_styles.get(&TextStyle::Small).unwrap().size;

        table(
            ui,
            row_height,
            filtered_events.iter(),
            || {
                self.collector.clear();
            },
            |ui| {
                table_header(ui, Some(100.0), |ui| {
                    ui.label("Time");
                });
                table_header(ui, Some(80.0), |ui| {
                    level_menu_button(ui, &mut state.level_filter)
                });
                table_header(ui, Some(120.0), |ui| {
                    target_menu_button(ui, &mut state.target_filter)
                });
                table_header(ui, None, |ui| {
                    ui.label("Message");
                });
            },
            |ui, event| {
                table_cell(ui, 100.0, |ui| {
                    ui.colored_label(Color32::GRAY, event.time.format_short())
                        .on_hover_text(event.time.format_detailed());
                });
                table_cell(ui, 80.0, |ui| {
                    ui.colored_label(event.level.to_color32(), event.level.as_str());
                });
                table_cell(ui, 120.0, |ui| {
                    ui.colored_label(Color32::GRAY, event.target.truncate_graphemes(18))
                        .on_hover_text(&event.target);
                });
                table_cell(ui, 120.0, |ui| {
                    let message = event.fields.get("message").unwrap();

                    ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                    ui.add(Label::new(message).wrap(false))
                        .on_hover_text(message);
                });
            },
        )
    }
}
