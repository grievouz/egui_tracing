mod collector;
mod event;

pub use self::collector::EventCollector;
pub use self::event::CapturedEvent;

pub fn collector() -> EventCollector {
    return EventCollector::default();
}
