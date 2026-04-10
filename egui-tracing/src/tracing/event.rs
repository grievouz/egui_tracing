use std::fmt::{self, Debug, Write};

use chrono::{DateTime, Local};
use tracing::field::{Field, Visit};
use tracing::{Event, Metadata};

#[derive(Debug, Clone)]
pub struct CollectedEvent {
    pub target: String,
    pub level: tracing::Level,
    pub message: Option<String>,
    pub fields: Vec<(String, String)>,
    pub time: DateTime<Local>,
    pub collapsed_summary: String,
}

impl CollectedEvent {
    pub fn new(event: &Event, meta: &Metadata) -> Self {
        let mut visitor = FieldVisitor {
            message: None,
            fields: Vec::new(),
        };
        event.record(&mut visitor);

        let collapsed_summary = Self::build_summary(&visitor.message, &visitor.fields);

        CollectedEvent {
            level: meta.level().to_owned(),
            time: Local::now(),
            target: meta.target().to_owned(),
            message: visitor.message,
            fields: visitor.fields,
            collapsed_summary,
        }
    }

    fn build_summary(message: &Option<String>, fields: &[(String, String)]) -> String {
        let mut summary = String::new();
        if let Some(msg) = message {
            for (i, line) in msg.trim().lines().enumerate() {
                if i > 0 {
                    summary.push(' ');
                }
                summary.push_str(line.trim());
            }
        }
        for (key, value) in fields {
            if !key.starts_with("log.") {
                let _ = write!(summary, ", {key}: {value}");
            }
        }
        summary
    }
}

struct FieldVisitor {
    message: Option<String>,
    fields: Vec<(String, String)>,
}

impl Visit for FieldVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_owned());
        } else {
            self.fields
                .push((field.name().to_owned(), value.to_owned()));
        }
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.fields
            .push((field.name().to_owned(), value.to_string()));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.fields
            .push((field.name().to_owned(), value.to_string()));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.fields
            .push((field.name().to_owned(), value.to_string()));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.fields
            .push((field.name().to_owned(), value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        let formatted = format!("{value:?}");
        if field.name() == "message" {
            self.message = Some(formatted);
        } else {
            self.fields.push((field.name().to_owned(), formatted));
        }
    }
}
