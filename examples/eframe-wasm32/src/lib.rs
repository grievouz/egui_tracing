use egui_tracing::egui;
use egui_tracing::tracing::collector::EventCollector;
use egui_tracing::tracing_subscriber;
use egui_tracing::tracing_subscriber::layer::SubscriberExt;
use egui_tracing::tracing_subscriber::util::SubscriberInitExt;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    let collector = egui_tracing::EventCollector::default();
    tracing_subscriber::registry()
        .with(collector.clone())
        .init();

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "eframe-canvas",
                web_options,
                Box::new(|_cc| Ok(Box::new(MyApp::new(collector)))),
            )
            .await
            .expect("failed to start eframe");
    });
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
            ui.add(egui_tracing::Logs::new(self.collector.clone()))
        });
    }
}
