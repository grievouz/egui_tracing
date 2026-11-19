use egui_tracing::tracing_subscriber::prelude::*;
use egui_tracing::EventCollector;

// On wasm32 this must work without enabling egui_tracing's legacy `wasmbind`
// feature. In particular, collecting an event must not call the unsupported
// std::time::SystemTime clock on wasm32-unknown-unknown.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn collecting_an_event_records_a_local_timestamp() {
    let collector = EventCollector::default();
    let subscriber = egui_tracing::tracing_subscriber::registry().with(collector.clone());
    let before = chrono::Local::now();

    let _guard = subscriber.set_default();
    tracing::info!(answer = 42, "timestamp regression");

    let after = chrono::Local::now();
    let events = collector.events();
    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event.message.as_deref(), Some("timestamp regression"));
    assert!(event
        .fields
        .iter()
        .any(|(key, value)| key == "answer" && value == "42"));
    assert!(event.time >= before && event.time <= after);
    assert_eq!(event.time.offset(), after.offset());
}
