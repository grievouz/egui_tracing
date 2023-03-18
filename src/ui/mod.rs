mod color;
mod time;

use std::sync::{Arc, Mutex};

use egui::{Color32, Label, Response, RichText, TextStyle, Widget};
use tracing::Level;

use self::color::{ToColor32, DEBUG_COLOR, ERROR_COLOR, INFO_COLOR, TRACE_COLOR, WARN_COLOR};
use self::time::SpecificFormats;
use crate::string::Ellipse;
use crate::tracing::collector::EventCollector;

#[derive(Debug, Clone)]
struct LevelFilter {
    trace: bool,
    debug: bool,
    info: bool,
    warn: bool,
    error: bool,
}

impl Default for LevelFilter {
    fn default() -> Self {
        Self {
            trace: true,
            debug: true,
            info: true,
            warn: true,
            error: true,
        }
    }
}

impl LevelFilter {
    pub fn get(&self, level: Level) -> bool {
        match level {
            Level::TRACE => self.trace,
            Level::DEBUG => self.debug,
            Level::INFO => self.info,
            Level::WARN => self.warn,
            Level::ERROR => self.error,
        }
    }
}

struct Elements {
    jump_bottom_btn: Option<Response>,
}

#[derive(Default)]
struct LogsState {
    level_filter: LevelFilter,
}

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
        let mut elements = Elements {
            jump_bottom_btn: None,
        };

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.set_height(26.0);
                ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);

                ui.horizontal(|ui| {
                    ui.set_min_width(80.0);
                    ui.separator();
                    ui.add_space(2.0);
                    ui.label("Time");
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.set_min_width(40.0);
                    ui.separator();
                    ui.add_space(2.0);
                    ui.menu_button("Level", |ui| {
                        ui.label("Level Message Filter");
                        ui.add(egui::Checkbox::new(
                            &mut state.level_filter.trace,
                            RichText::new("TRACE").color(TRACE_COLOR),
                        ));
                        ui.add(egui::Checkbox::new(
                            &mut state.level_filter.debug,
                            RichText::new("DEBUG").color(DEBUG_COLOR),
                        ));
                        ui.add(egui::Checkbox::new(
                            &mut state.level_filter.info,
                            RichText::new("INFO").color(INFO_COLOR),
                        ));
                        ui.add(egui::Checkbox::new(
                            &mut state.level_filter.warn,
                            RichText::new("WARN").color(WARN_COLOR),
                        ));
                        ui.add(egui::Checkbox::new(
                            &mut state.level_filter.error,
                            RichText::new("ERROR").color(ERROR_COLOR),
                        ));
                    });
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.set_min_width(100.0);
                    ui.separator();
                    ui.add_space(2.0);
                    ui.label("Target");
                });

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.separator();
                    ui.add_space(2.0);
                    ui.label(format!("Message ({:})", events.len()));
                });

                ui.add_space(ui.available_width() - 130.0);

                elements.jump_bottom_btn =
                    Some(ui.button("To Bottom").on_hover_text("Scroll to Bottom"));

                ui.separator();

                if ui.button("Clear").on_hover_text("Clear Events").clicked() {
                    self.collector.clear();
                }
            });

            ui.separator();

            let filtered_events = events
                .iter()
                .filter(|event| state.level_filter.get(event.level))
                .collect::<Vec<_>>();

            let row_height = ui.style().spacing.interact_size.y
                + ui.style().text_styles.get(&TextStyle::Small).unwrap().size;

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show_rows(ui, row_height, filtered_events.len(), |ui, range| {
                    for event in filtered_events.iter().skip(range.start).take(range.len()) {
                        ui.horizontal(|ui| {
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.set_min_width(80.0);
                                ui.add_space(5.0);
                                ui.colored_label(Color32::GRAY, event.time.format_short())
                                    .on_hover_text(event.time.format_detailed());
                            });

                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.set_min_width(40.0);
                                ui.add_space(5.0);
                                ui.colored_label(event.level.to_color32(), event.level.as_str());
                            });

                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.set_min_width(100.0);
                                ui.add_space(5.0);
                                ui.colored_label(
                                    Color32::GRAY,
                                    event.target.truncate_graphemes(14),
                                )
                                .on_hover_text(&event.target);
                            });

                            ui.add_space(5.0);

                            let message = event.fields.get("message").unwrap();
                            ui.horizontal(|ui| {
                                ui.add_space(5.0);
                                ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                                ui.add(Label::new(message).wrap(false))
                                    .on_hover_text(message);
                            });
                        });

                        ui.separator();
                    }

                    if elements.jump_bottom_btn.unwrap().clicked() {
                        ui.scroll_to_rect(
                            egui::Rect {
                                min: egui::Pos2 { x: 0.0, y: 0.0 },
                                max: egui::Pos2 {
                                    x: f32::MAX,
                                    y: f32::MAX,
                                },
                            },
                            Some(egui::Align::Max),
                        );
                    }
                });
        })
        .response
    }
}
