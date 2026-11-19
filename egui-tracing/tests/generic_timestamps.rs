use egui_tracing::tracing::{CollectedEvent, GenericCollectedEvent};
use egui_tracing::tracing_subscriber::prelude::*;
use egui_tracing::{EventCollector, GenericEventCollector, GenericLogs, Logs, Timestamp};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn chrono_api_remains_concrete() {
    // No type annotations needed for constructors, Default, or subsequent clones.
    let collector = EventCollector::new();
    let default_collector = EventCollector::default();
    let _: egui_tracing::tracing::collector::EventCollector = collector.clone();
    let _: egui_tracing::tracing::EventCollector = default_collector;
    let _: imbl::Vector<CollectedEvent> = collector.events();
    let _: Logs = Logs::new(collector.clone());
    let _: egui_tracing::ui::Logs = Logs::new(collector);
    let _constructor = CollectedEvent::new;
    let _module_constructor = egui_tracing::tracing::event::CollectedEvent::new;

    // Public struct literals, destructuring, and chrono methods still work.
    let event = CollectedEvent {
        target: "compatibility".into(),
        level: tracing::Level::INFO,
        message: None,
        fields: Vec::new(),
        time: chrono::Local::now(),
        collapsed_summary: String::new(),
    };
    let _: chrono::DateTime<chrono::Local> = event.time;
    let CollectedEvent { time, .. } = event;
    assert_eq!(time.timestamp(), time.naive_utc().and_utc().timestamp());
    assert_eq!(
        Timestamp::format_short(&time),
        time.format("%H:%M:%S%.3f").to_string()
    );
    assert_eq!(
        Timestamp::format_detailed(&time),
        time.format("%Y-%m-%dT%H:%M:%S%.f%:z").to_string()
    );
}

// Deliberately neither Default nor Copy: the generic core must not require them.
#[derive(Clone, Debug, PartialEq)]
struct TestTimestamp(u64);

impl Timestamp for TestTimestamp {
    fn now() -> Self {
        Self(42)
    }
    fn format_short(&self) -> String {
        format!("tick {}", self.0)
    }
    fn format_detailed(&self) -> String {
        self.format_short()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn custom_timestamp_flows_through_collector_and_widget() {
    let collector = GenericEventCollector::<TestTimestamp>::default().with_max_events(Some(2));
    let clone = collector.clone();
    let subscriber = egui_tracing::tracing_subscriber::registry().with(clone);
    let _guard = subscriber.set_default();
    for n in 0..3 {
        tracing::info!(n, "generic event");
    }
    let events: imbl::Vector<GenericCollectedEvent<TestTimestamp>> = collector.events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].time, TestTimestamp(42));
    assert_eq!(events[0].time.format_short(), "tick 42");
    assert!(events[1]
        .fields
        .iter()
        .any(|(key, value)| key == "n" && value == "2"));
    assert_eq!(events[1].collapsed_summary, "generic event, n: 2");
    assert_widget(GenericLogs::new(collector.clone()));
    #[cfg(feature = "egui-default")]
    {
        let ctx = egui_tracing::egui::Context::default();
        let mut output = ctx.run_ui(Default::default(), |ui| {
            ui.add(GenericLogs::new(collector.clone()));
        });
        output.textures_delta.clear();
    }
    collector.clear();
    assert!(collector.events().is_empty());
}

fn assert_widget(_: impl egui_tracing::egui::Widget) {}

#[cfg(feature = "time")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn time_and_chrono_collect_together() {
    use egui_tracing::{TimeEventCollector, TimeLogs};
    let chrono_collector = EventCollector::new();
    let time_collector = TimeEventCollector::new();
    let _: GenericEventCollector<time::OffsetDateTime> = time_collector.clone();
    let subscriber = egui_tracing::tracing_subscriber::registry()
        .with(chrono_collector.clone())
        .with(time_collector.clone());
    let before = time::OffsetDateTime::now_utc();
    let _guard = subscriber.set_default();
    tracing::info!(answer = 42, "both backends");
    let after = time::OffsetDateTime::now_utc();
    let events: imbl::Vector<egui_tracing::tracing::TimeCollectedEvent> = time_collector.events();
    assert_eq!(events.len(), 1);
    assert_eq!(chrono_collector.events().len(), 1);
    let timestamp = events[0].time;
    assert!(timestamp >= before && timestamp <= after);
    assert!(timestamp.offset().is_utc());
    assert!(timestamp.format_short().ends_with(" Z"));
    assert!(timestamp.format_detailed().ends_with("+00:00:00"));
    assert_widget(TimeLogs::new(time_collector.clone()));
    #[cfg(feature = "egui-default")]
    {
        let ctx = egui_tracing::egui::Context::default();
        let mut output = ctx.run_ui(Default::default(), |ui| {
            // Distinct backend state can coexist in the same frame.
            ui.push_id("chrono", |ui| {
                ui.add(Logs::new(chrono_collector.clone()));
            });
            ui.push_id("time", |ui| {
                ui.add(TimeLogs::new(time_collector.clone()));
            });
        });
        output.textures_delta.clear();
    }
}

#[cfg(feature = "time")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn time_formatting_preserves_precision_and_explicit_offsets() {
    let utc = time::OffsetDateTime::from_unix_timestamp_nanos(1_000_000_123).unwrap();
    assert_eq!(utc.format_short(), "00:00:01.000 Z");
    assert_eq!(
        utc.format_detailed(),
        "1970-01-01T0:00:01.000000123+00:00:00"
    );
    let shifted = utc.to_offset(time::UtcOffset::from_hms(5, 30, 0).unwrap());
    assert_eq!(shifted.format_short(), "05:30:01.000 +05:30:00");
    assert_eq!(shifted.unix_timestamp_nanos(), utc.unix_timestamp_nanos());
    assert!(shifted.format_detailed().ends_with("+05:30:00"));
    // UTC capture/formatting has no local-offset failure or DST dependency.
    let negative = time::OffsetDateTime::from_unix_timestamp_nanos(-1).unwrap();
    assert!(negative.format_detailed().contains(".999999999"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn concurrent_capture_shares_the_buffer() {
    let collector = GenericEventCollector::<TestTimestamp>::new();
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let collector = collector.clone();
            std::thread::spawn(move || {
                let subscriber = egui_tracing::tracing_subscriber::registry().with(collector);
                let _guard = subscriber.set_default();
                for _ in 0..10 {
                    tracing::info!("worker");
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(collector.events().len(), 40);
}

#[cfg(all(feature = "time", not(target_arch = "wasm32")))]
#[test]
fn time_capture_works_on_worker_threads() {
    let collector = egui_tracing::TimeEventCollector::new();
    let clone = collector.clone();
    std::thread::spawn(move || {
        let subscriber = egui_tracing::tracing_subscriber::registry().with(clone);
        let _guard = subscriber.set_default();
        tracing::info!("UTC worker");
    })
    .join()
    .unwrap();
    assert!(collector.events()[0].time.offset().is_utc());
}
