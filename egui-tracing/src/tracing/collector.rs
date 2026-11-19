use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use imbl::Vector;
use tracing::{Event, Level, Subscriber};
#[cfg(feature = "log")]
use tracing_log::NormalizeEvent;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

use super::event::GenericCollectedEvent;
use crate::Timestamp;

#[derive(Clone, Debug)]
pub enum AllowedTargets {
    All,
    Selected(Vec<String>),
}

#[derive(Debug, Clone)]
/// A tracing layer collecting events with the selected timestamp type.
///
/// This uses static dispatch; cloning a collector shares its event buffer.
pub struct GenericEventCollector<T: Timestamp> {
    allowed_targets: AllowedTargets,
    max_level: Level,
    max_events: Option<usize>,
    events: Arc<Mutex<Vector<GenericCollectedEvent<T>>>>,
    pending: Arc<Mutex<Vec<GenericCollectedEvent<T>>>>,
    generation: Arc<AtomicU64>,
}

/// The default collector, preserving chrono local timestamps.
pub type EventCollector = GenericEventCollector<chrono::DateTime<chrono::Local>>;

/// A collector capturing UTC timestamps using `time`.
#[cfg(feature = "time")]
pub type TimeEventCollector = GenericEventCollector<::time::OffsetDateTime>;

impl<T: Timestamp> GenericEventCollector<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_level(self, max_level: Level) -> Self {
        Self { max_level, ..self }
    }

    pub fn with_max_events(self, max_events: Option<usize>) -> Self {
        Self {
            max_events,
            ..self
        }
    }

    pub fn allowed_targets(self, allowed_targets: AllowedTargets) -> Self {
        Self {
            allowed_targets,
            ..self
        }
    }

    pub fn max_level(&self) -> Level {
        self.max_level
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

    pub fn events(&self) -> Vector<GenericCollectedEvent<T>> {
        self.drain_pending();
        self.events.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        {
            let mut pending = self.pending.lock().unwrap();
            pending.clear();
        }
        let mut events = self.events.lock().unwrap();
        *events = Vector::new();
        self.generation.fetch_add(1, Ordering::Relaxed);
    }

    /// Flush pending events into the main vector, bumping generation once.
    pub fn drain_pending(&self) {
        let mut pending = self.pending.lock().unwrap();
        if pending.is_empty() {
            return;
        }
        let mut events = self.events.lock().unwrap();
        for event in pending.drain(..) {
            events.push_back(event);
        }
        if let Some(max) = self.max_events {
            while events.len() > max {
                events.pop_front();
            }
        }
        self.generation.fetch_add(1, Ordering::Relaxed);
    }

    fn collect(&self, event: GenericCollectedEvent<T>) {
        if event.level <= self.max_level {
            let should_collect = match self.allowed_targets {
                AllowedTargets::All => true,
                AllowedTargets::Selected(ref selection) => selection
                    .iter()
                    .any(|target| event.target.starts_with(target)),
            };
            if should_collect {
                self.pending.lock().unwrap().push(event);
            }
        }
    }
}

impl<T: Timestamp> Default for GenericEventCollector<T> {
    fn default() -> Self {
        Self {
            allowed_targets: AllowedTargets::All,
            max_events: None,
            events: Arc::new(Mutex::new(Vector::new())),
            pending: Arc::new(Mutex::new(Vec::new())),
            generation: Arc::new(AtomicU64::new(0)),
            max_level: Level::DEBUG,
        }
    }
}

impl<S, T: Timestamp> Layer<S> for GenericEventCollector<T>
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

        self.collect(GenericCollectedEvent::new(event, meta));
    }
}
