use egui_tracing::tracing::collector::EventCollector;
use egui_tracing::tracing_subscriber::layer::SubscriberExt;
use egui_tracing::tracing_subscriber::util::SubscriberInitExt;
use egui_tracing::{egui, tracing_subscriber, Labels};
use tracing::Level;

fn portuguese_labels() -> Labels {
    let mut l = Labels::default();
    l.time = "Tempo".into();
    l.level = "Nivel".into();
    l.target = "Alvo".into();
    l.message = "Mensagem".into();
    l.clear = "Limpar".into();
    l.to_bottom = "Até Fundo".into();
    l.close = "Fechar".into();
    l.event_details = "Detalhes do Evento".into();
    l.message_too_long = "(mensagem muito longa)".into();
    l.level_filter = "Filtro de Nivel".into();
    l.target_filter = "Filtro de Alvo".into();
    l.add = "Adicionar".into();
    l.delete = "Excluir".into();
    l.target_placeholder = "exemplo: eframe::*".into();
    l
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Language {
    English,
    Portuguese,
}

fn main() {
    let collector = egui_tracing::EventCollector::default()
        .with_max_events(None)
        .with_max_level(Level::TRACE);
    tracing_subscriber::registry()
        .with(collector.clone())
        .init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            resizable: Some(true),
            inner_size: Some(egui::vec2(800.0, 500.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "tracing",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(collector)))),
    )
    .unwrap();
}

pub struct MyApp {
    collector: EventCollector,
    language: Language,
    labels: Labels,
}

impl MyApp {
    fn new(collector: EventCollector) -> Self {
        Self {
            collector,
            language: Language::English,
            labels: Labels::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::bottom("status").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                let fps = 1.0 / ui.input(|i| i.stable_dt);
                ui.weak(format!("{fps:.0} FPS"));
                ui.separator();
                ui.weak(format!("{} collected", self.collector.len()));
                ui.separator();
                let before = self.language;
                egui::ComboBox::from_id_salt("lang")
                    .selected_text(format!("{:?}", self.language))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.language, Language::English, "English");
                        ui.selectable_value(&mut self.language, Language::Portuguese, "Português");
                    });
                if self.language != before {
                    self.labels = match self.language {
                        Language::English => Labels::default(),
                        Language::Portuguese => portuguese_labels(),
                    };
                }
            });
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add(
                egui_tracing::Logs::new(self.collector.clone())
                    .with_labels(self.labels.clone()),
            );
        });
    }
}
