#![warn(clippy::all, clippy::cargo)]

mod string;
mod time;
pub mod tracing;
pub mod ui;

pub use self::tracing::EventCollector;
pub use self::ui::Logs;

#[cfg(feature = "reexport")]
pub use egui;
#[cfg(feature = "reexport")]
pub use tracing_subscriber;
