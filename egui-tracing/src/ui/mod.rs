mod color;
mod components;
mod state;

use std::sync::{Arc, Mutex};

use egui::{Label, Response, TextStyle, TextWrapMode, Widget};
use egui_extras::{Column, TableBuilder};
use globset::GlobSetBuilder;

use self::color::ToColor32;
use self::components::level_menu_button::LevelMenuButton;
use self::components::target_menu_button::TargetMenuButton;
use self::state::LogsState;
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
        #[cfg(debug_assertions)]
        ui.ctx().style_mut_of(ui.theme(), |style| {
            style.debug.warn_if_rect_changes_id = false;
        });

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

        let row_height = ui
            .style()
            .text_styles
            .get(&TextStyle::Small)
            .unwrap()
            .size
            + 4.0;

        let response = ui
            .vertical(|ui| {
                // Toolbar
                ui.horizontal(|ui| {
                    if ui
                        .button("To Bottom")
                        .on_hover_text("Scroll to Bottom")
                        .clicked()
                    {
                        ui.scroll_to_rect(
                            egui::Rect::from_min_max(
                                egui::pos2(0.0, 0.0),
                                egui::pos2(f32::MAX, f32::MAX),
                            ),
                            Some(egui::Align::Max),
                        );
                    }

                    ui.separator();

                    if ui.button("Clear").on_hover_text("Clear Events").clicked() {
                        self.collector.clear();
                    }
                });

                ui.separator();

                // Table
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .stick_to_bottom(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(100.0).at_least(60.0))
                    .column(Column::initial(60.0).at_least(40.0))
                    .column(Column::initial(140.0).at_least(60.0))
                    .column(Column::remainder().at_least(100.0).clip(true));

                table
                    .header(row_height, |mut header| {
                        header.col(|ui| {
                            ui.label("Time");
                        });
                        header.col(|ui| {
                            LevelMenuButton::default()
                                .state(&mut state.level_filter)
                                .show(ui);
                        });
                        header.col(|ui| {
                            TargetMenuButton::default()
                                .state(&mut state.target_filter)
                                .show(ui);
                        });
                        header.col(|ui| {
                            ui.label("Message");
                        });
                    })
                    .body(|body| {
                        body.rows(row_height, filtered_events.len(), |mut row| {
                            let event = filtered_events[row.index()];

                            row.col(|ui| {
                                ui.label(event.time.format_short())
                                    .on_hover_text(event.time.format_detailed());
                            });

                            row.col(|ui| {
                                ui.colored_label(
                                    event.level.to_color32(),
                                    event.level.as_str(),
                                );
                            });

                            row.col(|ui| {
                                ui.add(
                                    Label::new(&event.target)
                                        .wrap_mode(TextWrapMode::Truncate),
                                )
                                .on_hover_text(&event.target);
                            });

                            row.col(|ui| {
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
                                        log_message
                                            .push_str(&format!("\n {key}: {value}"));
                                    } else {
                                        short_message
                                            .push_str(&format!(", {key}: {value}"));
                                        complete_message
                                            .push_str(&format!("\n {key}: {value}"));
                                    }
                                }

                                complete_message.push_str("\n\n");
                                complete_message.push_str(&log_message);

                                ui.add(
                                    Label::new(short_message)
                                        .wrap_mode(TextWrapMode::Truncate),
                                )
                                .on_hover_text(complete_message);
                            });
                        });
                    });
            })
            .response;

        response
    }
}
