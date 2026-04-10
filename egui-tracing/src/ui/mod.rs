mod color;
mod components;
mod state;

use std::hash::{Hash, Hasher};
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

        self.collector.drain_pending();
        let generation = self.collector.generation();
        let filter_hash = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            state.target_filter.hash(&mut hasher);
            state.level_filter.hash() ^ hasher.finish()
        };

        {
            let LogsState {
                ref level_filter,
                ref target_filter,
                ref mut cache,
                ..
            } = *state;
            cache.rebuild_glob_set(&target_filter.targets);
            if cache.needs_update(generation, filter_hash) {
                let events = self.collector.events();
                cache.update(&events, generation, filter_hash, level_filter);
            }
        }

        let filtered_count = state.cache.len();

        let text_height = ui.style().text_styles.get(&TextStyle::Small).unwrap().size;
        let row_height = text_height + 18.0;
        let header_height = text_height + 18.0;

        // Detail panel pinned to bottom — rendered first to reserve space
        if let Some(selected) = state.selected_row {
            if selected < filtered_count {
                let event = state.cache.get(selected).clone();
                let mut close = false;
                egui::Panel::bottom(ui.id().with("detail_panel"))
                    .resizable(false)
                    .frame(egui::Frame::side_top_panel(ui.style()))
                    .show_inside(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.strong("Event Details");
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("Close").clicked() {
                                        close = true;
                                    }
                                },
                            );
                        });
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Time:").weak());
                            ui.label(event.time.format_short());
                            ui.separator();
                            ui.label(RichText::new("Level:").weak());
                            ui.colored_label(
                                event.level.to_color32(),
                                event.level.as_str(),
                            );
                            ui.separator();
                            ui.label(RichText::new("Target:").weak());
                            ui.label(&event.target);
                        });
                        if let Some(msg) = &event.message {
                            ui.add_space(4.0);
                            let trimmed = msg.trim();
                            let max_msg_lines = 25;
                            let lines: Vec<&str> = trimmed.lines().collect();
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
                        if !event.fields.is_empty() {
                            ui.add_space(2.0);
                            ui.separator();
                            for (key, value) in &event.fields {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!("{key}:")).weak(),
                                    );
                                    ui.label(value);
                                });
                            }
                        }
                    });
                if close {
                    state.selected_row = None;
                }
            } else {
                state.selected_row = None;
            }
        }

        // Log table fills remaining space — uniform row heights (O(1) scroll)
        let response = ui.vertical(|ui| {
            let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .stick_to_bottom(true)
            .sense(egui::Sense::click())
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::initial(100.0).at_least(60.0))
            .column(Column::initial(60.0).at_least(40.0))
            .column(Column::initial(140.0).at_least(60.0))
            .column(Column::remainder().at_least(100.0).clip(true));

            if state.scroll_to_bottom && filtered_count > 0 {
                table = table
                    .animate_scrolling(false)
                    .scroll_to_row(filtered_count - 1, Some(egui::Align::BOTTOM));
                state.scroll_to_bottom = false;
            }

            table.header(header_height, |mut header| {
                header.col(|ui| {
                    ui.label("Time");
                    let bottom = ui.max_rect().bottom();
                    let full_width = ui.ctx().content_rect().x_range();
                    ui.painter().hline(
                        full_width,
                        bottom,
                        ui.visuals().widgets.noninteractive.bg_stroke,
                    );
                });
                header.col(|ui| {
                    LevelMenuButton::default()
                        .state(&mut state.level_filter)
                        .max_level(self.collector.max_level())
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
                            if ui.button("Clear").clicked() {
                                self.collector.clear();
                            }
                            if ui.button("To Bottom").clicked() {
                                state.scroll_to_bottom = true;
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
                let selected = state.selected_row;
                let mut clicked_row = None;

                body.rows(row_height, filtered_count, |mut row| {
                    row.set_hovered(false);
                    let idx = row.index();
                    let is_selected = Some(idx) == selected;
                    let event = state.cache.get(idx);

                    let mut row_clicked = false;

                    row.col(|ui| {
                        ui.style_mut().interaction.selectable_labels = false;
                        if is_selected {
                            ui.visuals_mut().override_text_color =
                                Some(ui.visuals().strong_text_color());
                        }
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
                        if is_selected {
                            ui.visuals_mut().override_text_color =
                                Some(ui.visuals().strong_text_color());
                        }
                        ui.add(
                            Label::new(&event.target)
                                .wrap_mode(TextWrapMode::Truncate),
                        );
                    });

                    row.col(|ui| {
                        if is_selected {
                            ui.visuals_mut().override_text_color =
                                Some(ui.visuals().strong_text_color());
                        }
                        ui.add(
                            Label::new(&event.collapsed_summary)
                                .wrap_mode(TextWrapMode::Truncate),
                        );
                    });

                    if row_clicked || row.response().clicked() {
                        clicked_row = Some(idx);
                    }
                });

                if let Some(idx) = clicked_row {
                    state.selected_row = if Some(idx) == selected {
                        None
                    } else {
                        Some(idx)
                    };
                }
            });
        }).response;

        response
    }
}
