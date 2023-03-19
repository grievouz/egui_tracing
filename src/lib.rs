#![warn(clippy::all, clippy::cargo)]

mod string;
pub mod tracing;
pub mod ui;

pub use self::tracing::EventCollector;
pub use self::ui::Logs;
