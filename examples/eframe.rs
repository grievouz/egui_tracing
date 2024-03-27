use egui_tracing_rs::tracing::collector::EventCollector;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    let collector = EventCollector::default();
    tracing_subscriber::registry()
        .with(collector.clone())
        .init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(egui::vec2(800.0, 500.0)),
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
        Self { collector }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui_tracing_rs::Logs::new(self.collector.clone()))
        });
    }
}
