use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use imbl::Vector;
use tracing::{Event, Level, Subscriber};
#[cfg(feature = "log")]
use tracing_log::NormalizeEvent;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

use super::event::CollectedEvent;

#[derive(Clone, Debug)]
pub enum AllowedTargets {
    All,
    Selected(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct EventCollector {
    allowed_targets: AllowedTargets,
    level: Level,
    max_events: Option<usize>,
    events: Arc<Mutex<Vector<CollectedEvent>>>,
    generation: Arc<AtomicU64>,
}

impl EventCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(self, level: Level) -> Self {
        Self { level, ..self }
    }

    pub fn with_max_events(self, max_events: Option<usize>) -> Self {
        Self {
            max_events: max_events,
            ..self
        }
    }

    pub fn allowed_targets(self, allowed_targets: AllowedTargets) -> Self {
        Self {
            allowed_targets,
            ..self
        }
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::Relaxed)
    }

    pub fn len(&self) -> usize {
        self.events.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn events(&self) -> Vector<CollectedEvent> {
        self.events.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        let mut events = self.events.lock().unwrap();
        *events = Vector::new();
        self.generation.fetch_add(1, Ordering::Relaxed);
    }

    fn collect(&self, event: CollectedEvent) {
        if event.level <= self.level {
            let should_collect = match self.allowed_targets {
                AllowedTargets::All => true,
                AllowedTargets::Selected(ref selection) => selection
                    .iter()
                    .any(|target| event.target.starts_with(target)),
            };
            if should_collect {
                let mut events = self.events.lock().unwrap();
                events.push_back(event);
                if let Some(max) = self.max_events {
                    while events.len() > max {
                        events.pop_front();
                    }
                }
                self.generation.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

impl Default for EventCollector {
    fn default() -> Self {
        Self {
            allowed_targets: AllowedTargets::All,
            max_events: None,
            events: Arc::new(Mutex::new(Vector::new())),
            generation: Arc::new(AtomicU64::new(0)),
            level: Level::DEBUG,
        }
    }
}

impl<S> Layer<S> for EventCollector
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        #[cfg(feature = "log")]
        let normalized_meta = event.normalized_metadata();
        #[cfg(feature = "log")]
        let meta = normalized_meta.as_ref().unwrap_or_else(|| event.metadata());
        #[cfg(not(feature = "log"))]
        let meta = event.metadata();

        self.collect(CollectedEvent::new(event, meta));
    }
}
