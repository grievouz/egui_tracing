use egui_tracing::tracing::collector::EventCollector;
use egui_tracing::tracing_subscriber::layer::SubscriberExt;
use egui_tracing::tracing_subscriber::util::SubscriberInitExt;
use egui_tracing::{egui, tracing_subscriber};
use tracing::Level;

fn main() {
    let collector = egui_tracing::EventCollector::default()
        .with_max_events(None)
        .with_level(Level::TRACE);
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
}

impl MyApp {
    fn new(collector: EventCollector) -> Self {
        Self { collector }
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
            });
        });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add(egui_tracing::Logs::new(self.collector.clone()));
        });
    }
}
