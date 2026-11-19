#![warn(clippy::all, clippy::cargo)]

mod string;
mod timestamp;
pub mod tracing;
pub mod ui;

pub use egui;
pub use tracing_subscriber;

pub use self::timestamp::Timestamp;
pub use self::tracing::{EventCollector, GenericEventCollector};
#[cfg(feature = "time")]
pub use self::tracing::TimeEventCollector;
pub use self::ui::labels::Labels;
pub use self::ui::{GenericLogs, Logs};
#[cfg(feature = "time")]
pub use self::ui::TimeLogs;
