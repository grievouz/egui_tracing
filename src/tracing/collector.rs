use chrono::Local;
use std::{
    collections::BTreeMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};
use tracing::{
    field::{Field, Visit},
    Event, Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

use super::event::CapturedEvent;

#[derive(Debug, Clone)]
pub struct EventCollector {
    events: Arc<Mutex<Vec<CapturedEvent>>>,
}

impl EventCollector {
    pub fn events(&self) -> Vec<CapturedEvent> {
        return self.events.lock().unwrap().clone();
    }

    pub fn clear(&self) {
        let mut events = self.events.lock().unwrap();
        *events = Vec::new();
    }

    fn collect(&self, event: CapturedEvent) {
        self.events.lock().unwrap().push(event);
    }
}

impl Default for EventCollector {
    fn default() -> Self {
        return Self {
            events: Arc::new(Mutex::new(Vec::new())),
        };
    }
}

impl<S> Layer<S> for EventCollector
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let meta = event.metadata();
        let mut fields = BTreeMap::new();
        let mut visitor = FieldVisitor(&mut fields);
        event.record(&mut visitor);
        self.collect(CapturedEvent {
            level: meta.level().to_owned().into(),
            time: Local::now(),
            target: meta.target().into(),
            fields,
        });
    }
}

struct FieldVisitor<'a>(&'a mut BTreeMap<String, String>);

impl<'a> Visit for FieldVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_string(), format!("{:?}", value));
    }
}
