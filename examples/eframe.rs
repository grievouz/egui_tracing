use egui_tracing::EventCollector;
use tracing::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let collector = egui_tracing::collector();
    tracing_subscriber::registry()
        .with(collector.clone())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(egui::vec2(600.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "tracing",
        options,
        Box::new(|_cc| Box::new(MyApp::new(collector))),
    )
    .unwrap();
}

pub struct MyApp {
    collector: EventCollector,
}

impl MyApp {
    fn new(collector: EventCollector) -> Self {
        return Self { collector };
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        warn!("test");
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_tracing::ui(&self.collector, ui);
        });
    }
}
