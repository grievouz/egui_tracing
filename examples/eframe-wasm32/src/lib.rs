use egui_tracing::egui;
use egui_tracing::tracing::collector::EventCollector;
#[cfg(target_arch = "wasm32")]
use egui_tracing::tracing_subscriber;
#[cfg(target_arch = "wasm32")]
use egui_tracing::tracing_subscriber::layer::SubscriberExt;
#[cfg(target_arch = "wasm32")]
use egui_tracing::tracing_subscriber::util::SubscriberInitExt;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    let collector = egui_tracing::EventCollector::default();
    tracing_subscriber::registry()
        .with(collector.clone())
        .init();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("eframe-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                canvas,
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
    #[cfg(target_arch = "wasm32")]
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
