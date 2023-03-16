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

use super::event::CollectedEvent;

#[derive(Debug, Clone)]
pub struct EventCollector {
    events: Arc<Mutex<Vec<CollectedEvent>>>,
}

impl EventCollector {
    pub fn events(&self) -> Vec<CollectedEvent> {
        return self.events.lock().unwrap().clone();
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
        let mut events = self.events.lock().unwrap();
        events.push(CollectedEvent {
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
