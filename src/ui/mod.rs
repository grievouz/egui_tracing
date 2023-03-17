mod color;
mod string;
mod time;

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use egui::mutex::Mutex;
use egui::{Color32, Id, Label, Response, RichText, TextStyle};
use lazy_static::lazy_static;
use tracing::Level;

use self::color::ToColor32;
use self::string::Ellipse;
use self::time::SpecificFormats;
use crate::tracing::{CapturedEvent, EventCollector};

lazy_static! {
    static ref STATES: Arc<Mutex<HashMap<Id, MemoryState>>> = Default::default();
}

#[derive(Debug, Clone)]
struct LevelFilter {
    trace: bool,
    debug: bool,
    info: bool,
    warn: bool,
    error: bool,
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

#[derive(Debug, Clone)]
struct MemoryState {
    pub level_filter: LevelFilter,
}

impl Default for MemoryState {
    fn default() -> Self {
        return Self {
            level_filter: LevelFilter {
                trace: true,
                debug: true,
                info: true,
                warn: true,
                error: true,
            },
        };
    }
}

struct Elements {
    jump_bottom_btn: Option<Response>,
}

pub fn ui(collector: &EventCollector, ui: &mut egui::Ui) {
    let events = collector.events();
    let id = ui.id();
    let mut mem_state = STATES
        .clone()
        .lock()
        .get(&id)
        .get_or_insert(&MemoryState::default())
        .clone();
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
                        &mut mem_state.level_filter.trace,
                        RichText::new("TRACE").color(Color32::WHITE),
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut mem_state.level_filter.debug,
                        RichText::new("DEBUG").color(Color32::WHITE),
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut mem_state.level_filter.info,
                        RichText::new("INFO").color(Color32::WHITE),
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut mem_state.level_filter.warn,
                        RichText::new("WARN").color(Color32::WHITE),
                    ));
                    ui.add(egui::Checkbox::new(
                        &mut mem_state.level_filter.error,
                        RichText::new("ERROR").color(Color32::WHITE),
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
                collector.clear();
            }
        });

        ui.separator();

        let filtered_events = events
            .iter()
            .filter(|event| mem_state.level_filter.get(event.level))
            .collect::<Vec<_>>();

        let row_height = ui.style().spacing.interact_size.y
            + ui.style().text_styles.get(&TextStyle::Small).unwrap().size;

        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show_rows(ui, row_height, events.len(), |ui, range| {
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
                            ui.colored_label(Color32::GRAY, event.target.truncate_graphemes(14))
                                .on_hover_text(&event.target);
                        });

                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.add_space(5.0);
                            ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                            ui.add(Label::new(event.fields.get("message").unwrap()).wrap(false));
                        });
                    });

                    ui.separator();
                }

                if elements.jump_bottom_btn.unwrap().clicked() {
                    ui.scroll_to_cursor(Some(egui::Align::Max));
                }
            });
    });

    STATES.lock().insert(id, mem_state);
}
