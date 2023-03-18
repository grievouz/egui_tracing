// #![warn(clippy::all, clippy::pedantic)]

mod string;
pub mod tracing;
pub mod ui;

pub use self::ui::Logs;
pub use self::tracing::EventCollector;