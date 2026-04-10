use egui_tracing::tracing::collector::EventCollector;
use egui_tracing::tracing_subscriber::layer::SubscriberExt;
use egui_tracing::tracing_subscriber::util::SubscriberInitExt;
use egui_tracing::ui::labels::TracingLabels;
use egui_tracing::{egui, tracing_subscriber};

#[derive(Debug, Clone, PartialEq)]
enum Language {
    English,
    BrazilianPortuguese,
}

fn get_labels_portuguese() -> TracingLabels {
    TracingLabels {
        time: "Tempo".to_string(),
        level: "Nivel".to_string(),
        target: "Alvo".to_string(),
        message: "Mensagem".to_string(),
        to_bottom: "Até Fundo".to_string(),
        scroll_to_bottom: "Desfilar até o Fundo".to_string(),
        clear: "Limpar".to_string(),
        clear_events: "Limpar Eventos".to_string(),
        level_filter: "Filtro de Nivel".to_string(),
        target_filter: "Filtro de Alvo".to_string(),
        add: "Adicionar".to_string(),
        delete: "Excluir".to_string(),
        example: "exemplo".to_string(),
    }
}

fn main() {
    let collector = egui_tracing::EventCollector::default();
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
    language: Language,
    logs: egui_tracing::Logs,
}

impl MyApp {
    fn new(collector: EventCollector) -> Self {
        Self {
            language: Language::English,
            logs: egui_tracing::Logs::new(collector),
        }
    }

    fn update_labels(&mut self) {
        *self.logs.labels_mut() = match self.language {
            Language::English => TracingLabels::default(),
            Language::BrazilianPortuguese => get_labels_portuguese(),
        };
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let language_before = self.language.clone();
            egui::ComboBox::from_label("Language")
                .selected_text(format!("{:?}", self.language))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.language, Language::English, "English");
                    ui.selectable_value(
                        &mut self.language,
                        Language::BrazilianPortuguese,
                        "BrazilianPortuguese",
                    );
                });
            if language_before != self.language {
                self.update_labels();
            }
            ui.add(&mut self.logs)
        });
    }
}
