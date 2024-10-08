mod color;
mod components;
mod state;

use std::sync::{Arc, Mutex};

use egui::{Label, Response, TextStyle, TextWrapMode, Widget};
use globset::GlobSetBuilder;

use self::color::ToColor32;
use self::components::common::CommonProps;
use self::components::constants;
use self::components::level_menu_button::LevelMenuButton;
use self::components::table::Table;
use self::components::table_cell::TableCell;
use self::components::table_header::TableHeader;
use self::components::target_menu_button::TargetMenuButton;
use self::state::LogsState;
use crate::string::Ellipse;
use crate::time::DateTimeFormatExt;
use crate::tracing::collector::EventCollector;
use crate::tracing::CollectedEvent;

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
        let state = ui.memory_mut(|mem| {
            let state_mem_id = ui.id();
            mem.data
                .get_temp_mut_or_insert_with(state_mem_id, || {
                    Arc::new(Mutex::new(LogsState::default()))
                })
                .clone()
        });
        let mut state = state.lock().unwrap();

        // TODO: cache the globset
        let glob = {
            let mut glob = GlobSetBuilder::new();
            for target in state.target_filter.targets.clone() {
                glob.add(target);
            }
            glob.build().unwrap()
        };

        let events = self.collector.events();
        let filtered_events = events
            .iter()
            .filter(|event| state.level_filter.get(event.level) && !glob.is_match(&event.target))
            .collect::<Vec<_>>();

        let row_height = constants::SEPARATOR_SPACING
            + ui.style().text_styles.get(&TextStyle::Small).unwrap().size;

        Table::default()
            .on_clear(|| {
                self.collector.clear();
            })
            .header(|ui| {
                TableHeader::default()
                    .common_props(CommonProps::new().min_width(100.0))
                    .children(|ui| {
                        ui.label("Time");
                    })
                    .show(ui);
                TableHeader::default()
                    .common_props(CommonProps::new().min_width(80.0))
                    .children(|ui| {
                        LevelMenuButton::default()
                            .state(&mut state.level_filter)
                            .show(ui);
                    })
                    .show(ui);
                TableHeader::default()
                    .common_props(CommonProps::new().min_width(120.0))
                    .children(|ui| {
                        TargetMenuButton::default()
                            .state(&mut state.target_filter)
                            .show(ui);
                    })
                    .show(ui);
                TableHeader::default()
                    .common_props(CommonProps::new().min_width(120.0))
                    .children(|ui| {
                        ui.label("Message");
                    })
                    .show(ui);
            })
            .row_height(row_height)
            .row(|ui, event: &CollectedEvent| {
                TableCell::default()
                    .common_props(CommonProps::new().min_width(100.0))
                    .children(|ui| {
                        ui.label(event.time.format_short())
                            .on_hover_text(event.time.format_detailed());
                    })
                    .show(ui);
                TableCell::default()
                    .common_props(CommonProps::new().min_width(80.0))
                    .children(|ui| {
                        ui.colored_label(event.level.to_color32(), event.level.as_str());
                    })
                    .show(ui);
                TableCell::default()
                    .common_props(CommonProps::new().min_width(120.0))
                    .children(|ui| {
                        ui.label(event.target.truncate_graphemes(18))
                            .on_hover_text(&event.target);
                    })
                    .show(ui);
                TableCell::default()
                    .common_props(CommonProps::new().min_width(120.0))
                    .children(|ui| {
                        let mut short_message = String::new();
                        let mut complete_message = String::new();
                        let mut log_message = String::new();

                        if let Some(msg) = event.fields.get("message") {
                            let msg = msg.trim();
                            short_message.push_str(msg);
                            complete_message.push_str(msg);
                        }

                        for (key, value) in &event.fields {
                            if key == "message" {
                                continue;
                            }
                            if key.starts_with("log.") {
                                log_message.push_str(&format!("\n {}: {}", key, value));
                            } else {
                                short_message.push_str(&format!(", {}: {}", key, value));
                                complete_message.push_str(&format!("\n {}: {}", key, value));
                            }
                        }

                        complete_message.push_str("\n\n");
                        complete_message.push_str(&log_message);

                        ui.add(Label::new(short_message).wrap_mode(TextWrapMode::Extend))
                            .on_hover_text(complete_message);
                    })
                    .show(ui);
            })
            .show(ui, filtered_events.iter())
    }
}
