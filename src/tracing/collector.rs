use chrono::Local;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use tracing::{
    field::{Field, Visit},
    Event, Subscriber,
    Level
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

use super::event::CapturedEvent;

#[derive(Clone, Debug)]
pub enum AllowedTargets {
    All,
    Selected(Vec<String>)
}

#[derive(Debug, Clone)]
pub struct EventCollector {
    allowed_targets: AllowedTargets,
    level: Level,
    events: Arc<Mutex<Vec<CapturedEvent>>>,
}

impl EventCollector {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(self, level: Level) -> Self {
        Self {
            level,
            ..self
        }
    }
    pub fn allowed_targets(self, allowed_targets: AllowedTargets) -> Self {
        Self {
            allowed_targets,
            ..self
        }
    }

    pub fn events(&self) -> Vec<CapturedEvent> {
        self.events.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        let mut events = self.events.lock().unwrap();
        *events = Vec::new();
    }

    fn collect(&self, event: CapturedEvent) {
        if event.level >= self.level {
            let should_collect = match self.allowed_targets {
                AllowedTargets::All => true,
                AllowedTargets::Selected(ref selection) => {
                    selection.iter().any(|target| { event.target.starts_with(target) })
                }
            };
            if should_collect {
                self.events.lock().unwrap().push(event);
            }
        }
    }
}

impl Default for EventCollector {
    fn default() -> Self {
        Self {
            allowed_targets: AllowedTargets::All,
            events: Arc::new(Mutex::new(Vec::new())),
            level: Level::TRACE  // capture everything by default.
        }
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
            level: meta.level().to_owned(),
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
