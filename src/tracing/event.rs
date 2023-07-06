use std::collections::BTreeMap;
use std::fmt::Debug;

use chrono::{DateTime, Local};
use tracing::field::{Field, Visit};
use tracing::{Event, Metadata};

#[derive(Debug, Clone)]
pub struct CollectedEvent {
    pub target: String,
    pub level: tracing::Level,
    pub fields: BTreeMap<String, String>,
    pub time: DateTime<Local>,
}

impl CollectedEvent {
    pub fn new(event: &Event, meta: &Metadata) -> Self {
        let mut fields = BTreeMap::new();
        event.record(&mut FieldVisitor(&mut fields));

        CollectedEvent {
            level: meta.level().to_owned(),
            time: Local::now(),
            target: meta.target().to_owned(),
            fields,
        }
    }
}

struct FieldVisitor<'a>(&'a mut BTreeMap<String, String>);

impl<'a> Visit for FieldVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_string(), format!("{:?}", value));
    }
}
