use chrono::{DateTime, Local};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug, Clone)]
pub struct CollectedEvent {
    pub target: String,
    pub level: tracing::Level,
    pub fields: BTreeMap<String, String>,
    pub time: DateTime<Local>,
}
