mod collector;
mod event;

pub use self::collector::EventCollector;

pub fn collector() -> EventCollector {
    return EventCollector::default();
}
