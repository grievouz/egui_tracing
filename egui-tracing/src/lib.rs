#![warn(clippy::all, clippy::cargo)]

mod string;
mod time;
pub mod tracing;
pub mod ui;

pub use egui;
pub use tracing_subscriber;

pub use self::tracing::EventCollector;
pub use self::ui::labels::Labels;
pub use self::ui::Logs;
