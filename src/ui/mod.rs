mod color;
mod components;
mod state;
mod time;

use std::sync::{Arc, Mutex};

use egui::{Color32, Label, Response, TextStyle, Widget};

use self::color::ToColor32;
use self::components::header_item::HeaderItem;
use self::components::level_menu_button::LevelFilterMenuButton;
use self::state::LogsState;
use self::time::SpecificFormats;
use crate::string::Ellipse;
use crate::tracing::collector::EventCollector;

struct Elements {
    jump_bottom_btn: Option<Response>,
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
            // header
            ui.horizontal(|ui| {
                ui.set_height(26.0);
                ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);

                ui.add(HeaderItem::default().set_min_width(80.0).set_child(|ui| {
                    ui.label("Time");
                }));

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.set_min_width(40.0);
                    ui.separator();
                    ui.add_space(2.0);
                    ui.add(LevelFilterMenuButton::new(&mut state.level_filter));
                });

                ui.add_space(5.0);

                ui.add(HeaderItem::default().set_min_width(100.0).set_child(|ui| {
                    ui.label("Target");
                }));

                ui.add_space(5.0);

                ui.add(HeaderItem::default().set_child(|ui| {
                    ui.label(format!("Message ({:})", events.len()));
                }));

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
