mod color;
mod components;
mod state;

use std::sync::{Arc, Mutex};

use egui::{Label, Response, RichText, TextStyle, TextWrapMode, Widget};
use egui_extras::{Column, TableBuilder};

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

        let generation = self.collector.generation();
        let filter_hash =
            state.level_filter.hash() ^ (state.target_filter.targets.len() as u64) << 32;

        {
            let LogsState {
                ref level_filter,
                ref target_filter,
                ref mut cache,
                ..
            } = *state;
            cache.rebuild_glob_set(&target_filter.targets);
            let events = self.collector.events();
            cache.update(&events, generation, filter_hash, level_filter);
        }

        let filtered_count = state.cache.filtered_events.len();

        let text_height = ui.style().text_styles.get(&TextStyle::Small).unwrap().size;
        let row_height = text_height + 18.0;
        let header_height = text_height + 18.0;


        let response = ui
            .vertical(|ui| {
                TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .stick_to_bottom(true)
                    .sense(egui::Sense::click())
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(100.0).at_least(60.0))
                    .column(Column::initial(60.0).at_least(40.0))
                    .column(Column::initial(140.0).at_least(60.0))
                    .column(Column::remainder().at_least(100.0).clip(true))
                    .header(header_height, |mut header| {
                        header.col(|ui| {
                            ui.label("Time");
                        });
                        header.col(|ui| {
                            LevelMenuButton::default()
                                .state(&mut state.level_filter)
                                .max_level(self.collector.level())
                                .show(ui);
                        });
                        header.col(|ui| {
                            TargetMenuButton::default()
                                .state(&mut state.target_filter)
                                .show(ui);
                        });
                        header.col(|ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("Clear").clicked() {
                                        self.collector.clear();
                                    }
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::Center),
                                        |ui| {
                                            ui.label("Message");
                                            ui.weak(format!("({})", filtered_count));
                                        },
                                    );
                                },
                            );
                        });
                    })
                    .body(|body| {
                        let expanded = state.expanded_row;
                        let expanded_height = if state.expanded_height > 0.0 {
                            state.expanded_height
                        } else {
                            row_height * 5.0 // initial estimate
                        };
                        let heights: Vec<f32> = (0..filtered_count)
                            .map(|i| {
                                if Some(i) == expanded {
                                    expanded_height
                                } else {
                                    row_height
                                }
                            })
                            .collect();

                        let mut clicked_row = None;
                        let mut measured_height = None;

                        body.heterogeneous_rows(heights.into_iter(), |mut row| {
                            row.set_hovered(false);
                            let idx = row.index();
                            let is_expanded = Some(idx) == expanded;
                            let event = &state.cache.filtered_events[idx];

                            let mut row_clicked = false;

                            row.col(|ui| {
                                ui.style_mut().interaction.selectable_labels = false;
                                let r = ui.label(event.time.format_short());
                                row_clicked |= r.clicked();
                            });

                            row.col(|ui| {
                                ui.style_mut().interaction.selectable_labels = false;
                                let r = ui.colored_label(
                                    event.level.to_color32(),
                                    event.level.as_str(),
                                );
                                row_clicked |= r.clicked();
                            });

                            row.col(|ui| {
                                ui.add(
                                    Label::new(&event.target)
                                        .wrap_mode(TextWrapMode::Truncate),
                                );
                            });

                            row.col(|ui| {
                                if is_expanded {
                                    // Expanded view: top-aligned with full content.
                                    // Add top padding to match the collapsed row's
                                    // vertical centering so the first line doesn't jump.
                                    let top_padding =
                                        (row_height - text_height) / 2.0
                                        - ui.spacing().item_spacing.y / 2.0;
                                    ui.with_layout(
                                        egui::Layout::top_down(egui::Align::LEFT),
                                        |ui| {
                                            ui.add_space(top_padding.max(0.0));
                                            if let Some(msg) = event.fields.get("message") {
                                                let trimmed = msg.trim();
                                                let max_msg_lines = 25;
                                                let lines: Vec<&str> =
                                                    trimmed.lines().collect();
                                                if lines.len() > max_msg_lines {
                                                    let truncated: String =
                                                        lines[..max_msg_lines].join("\n");
                                                    ui.add(
                                                        Label::new(truncated)
                                                            .wrap_mode(TextWrapMode::Wrap),
                                                    );
                                                    ui.weak("(message too long)");
                                                } else {
                                                    ui.add(
                                                        Label::new(trimmed)
                                                            .wrap_mode(TextWrapMode::Wrap),
                                                    );
                                                }
                                            }
                                            let has_fields = event
                                                .fields
                                                .keys()
                                                .any(|k| k != "message");
                                            if has_fields {
                                                ui.add_space(2.0);
                                                ui.separator();
                                                for (key, value) in &event.fields {
                                                    if key == "message" {
                                                        continue;
                                                    }
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            RichText::new(format!("{key}:"))
                                                                .weak(),
                                                        );
                                                        ui.label(value);
                                                    });
                                                }
                                            }
                                        },
                                    );
                                    // Measure actual content height for next frame
                                    measured_height = Some(
                                        ui.min_rect().height() + row_height - text_height,
                                    );
                                    ui.ctx().request_repaint();
                                } else {
                                    // Collapsed view: single-line summary
                                    let mut short_message = String::new();

                                    if let Some(msg) = event.fields.get("message") {
                                        for (i, line) in
                                            msg.trim().lines().enumerate()
                                        {
                                            if i > 0 {
                                                short_message.push(' ');
                                            }
                                            short_message.push_str(line.trim());
                                        }
                                    }

                                    for (key, value) in &event.fields {
                                        if key == "message" {
                                            continue;
                                        }
                                        if !key.starts_with("log.") {
                                            short_message.push_str(
                                                &format!(", {key}: {value}"),
                                            );
                                        }
                                    }

                                    ui.add(
                                        Label::new(short_message)
                                            .wrap_mode(TextWrapMode::Truncate),
                                    );
                                }
                            });

                            if row_clicked || row.response().clicked() {
                                clicked_row = Some(idx);
                            }
                        });

                        if let Some(h) = measured_height {
                            state.expanded_height = h;
                        }

                        if let Some(idx) = clicked_row {
                            state.expanded_height = 0.0;
                            state.expanded_row = if Some(idx) == expanded {
                                None
                            } else {
                                Some(idx)
                            };
                        }
                    });
            })
            .response;

        response
    }
}
