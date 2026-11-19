//! Collect tracing events and display them in egui.
//!
//! # Timestamp backends
//!
//! [`EventCollector`], [`tracing::CollectedEvent`], and [`Logs`] use
//! `chrono::DateTime<chrono::Local>`, preserving local timestamps and existing
//! constructors and struct literals.
//!
//! Enable the `time` feature to use UTC timestamps instead:
//!
//! ```toml
//! egui_tracing = { version = "0.4.0", features = ["time"] }
//! ```
//!
//! ```
//! # #[cfg(feature = "time")]
//! # {
//! let collector = egui_tracing::TimeEventCollector::new();
//! // Register collector.clone() as a tracing layer, then render with TimeLogs.
//! let logs = egui_tracing::TimeLogs::new(collector.clone());
//! # }
//! ```
//!
//! These aliases use [`GenericEventCollector<T>`] and [`GenericLogs<T>`] with
//! `time::OffsetDateTime`. Implement [`Timestamp`] on a local newtype for a custom
//! clock. Both backends can coexist; enabling a feature never changes an existing
//! field type, and `--all-features` is supported.
//!
//! The time backend captures UTC without local-offset discovery. The log widget
//! uses [`Timestamp::format_short`] for both the table and selected-event details;
//! time timestamps show milliseconds and are labelled with `Z`. Call
//! [`Timestamp::format_detailed`] directly for the date, numeric offset, and full
//! subsecond precision. Chrono retains its local-time formats. Both backends enable
//! their JavaScript clock support automatically on wasm32; `wasmbind` remains supported.
//!
//! Chrono remains an unconditional dependency, including with default features
//! disabled. Enabling `time` does not remove chrono or its audit notice.

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
