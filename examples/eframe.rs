use egui_tracing::tracing::collector::EventCollector;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    let collector = egui_tracing::EventCollector::default();
    tracing_subscriber::registry()
        .with(collector.clone())
        .init();

    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(egui::vec2(800.0, 500.0)),
        ..Default::default()
    };
    let c = collector.clone();
    eframe::run_native("tracing", options, Box::new(|_cc| Box::new(MyApp::new(c)))).unwrap();
    
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui_tracing::Logs::new(self.collector.clone()))
        });
    }
}
