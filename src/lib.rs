#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod tracing;
mod ui;

pub use crate::tracing::{collector, EventCollector};
pub use crate::ui::ui;
