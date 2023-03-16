mod color;
mod string;
mod time;

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use egui::mutex::Mutex;
use egui::{Color32, Id, Label, RichText};
use lazy_static::lazy_static;
use tracing::Level;

use self::color::ToColor32;
use self::string::Ellipse;
use self::time::SpecificFormats;
use crate::tracing::EventCollector;

lazy_static! {
    static ref STATES: Arc<Mutex<HashMap<Id, AppState>>> = Default::default();
}

#[derive(Debug, Clone)]
struct AppState {
    pub level_filter: BTreeMap<Level, bool>,
}

impl Default for AppState {
    fn default() -> Self {
        return Self {
            level_filter: BTreeMap::from([
                (Level::TRACE, true),
                (Level::DEBUG, true),
                (Level::INFO, true),
                (Level::WARN, true),
                (Level::ERROR, true),
            ]),
        };
    }
}

struct TransAppState {
    pub clicked_jump_bottom: bool,
}

pub fn ui(collector: &EventCollector, ui: &mut egui::Ui) {
    let events = collector.events();
    let id = ui.id();
    let mut state = STATES
        .clone()
        .lock()
        .get(&id)
        .get_or_insert(&AppState::default())
        .clone();
    let mut trans_state = TransAppState {
        clicked_jump_bottom: false,
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
                    state.level_filter.to_owned().keys().for_each(|level| {
                        ui.add(egui::Checkbox::new(
                            state.level_filter.get_mut(&level).unwrap(),
                            RichText::new(level.as_str()).color(level.to_color32()),
                        ));
                    });
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
            trans_state.clicked_jump_bottom = ui
                .button("To Bottom")
                .on_hover_text("Scroll to Bottom")
                .clicked();
            ui.separator();
            if ui.button("Clear").on_hover_text("Clear Events").clicked() {
                collector.clear();
            }
        });
        ui.separator();
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for event in events.iter().filter(|event| {
                    state
                        .level_filter
                        .get(&event.level)
                        .unwrap_or(&false)
                        .to_owned()
                }) {
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
                if trans_state.clicked_jump_bottom {
                    ui.scroll_to_cursor(Some(egui::Align::Max));
                }
            });
    });
    STATES.lock().insert(id, state);
}
