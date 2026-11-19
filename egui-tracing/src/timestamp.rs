use std::fmt::Debug;

use chrono::{DateTime, Local};

/// Timestamp captured by a [`crate::GenericEventCollector`].
///
/// Implement this on a local newtype to provide a custom clock or display format.
/// Collectors invoke `now` on the emitting thread, so implementations must support
/// concurrent capture. Formatting runs when the log UI is rendered.
///
/// The default [`DateTime<Local>`] implementation preserves chrono's local-time
/// capture and formatting. With the `time` feature, `time::OffsetDateTime`
/// captures UTC and labels that choice explicitly; it does not query the native
/// local offset (which may be unavailable in multithreaded programs).
///
/// There is no per-instance clock configuration or dynamic dispatch.
pub trait Timestamp: Clone + Debug + Send + Sync + 'static {
    /// Capture the event timestamp without dropping the event on offset failure.
    fn now() -> Self;

    /// Render a compact timestamp for the event table.
    fn format_short(&self) -> String;

    /// Render a full timestamp for callers that need more detail.
    ///
    /// The log widget currently uses [`Self::format_short`] in both the event
    /// table and the selected-event details panel.
    fn format_detailed(&self) -> String;
}

impl Timestamp for DateTime<Local> {
    fn now() -> Self {
        Local::now()
    }

    fn format_short(&self) -> String {
        self.format("%H:%M:%S%.3f").to_string()
    }

    fn format_detailed(&self) -> String {
        self.format("%Y-%m-%dT%H:%M:%S%.f%:z").to_string()
    }
}

#[cfg(feature = "time")]
impl Timestamp for time::OffsetDateTime {
    fn now() -> Self {
        Self::now_utc()
    }

    fn format_short(&self) -> String {
        let zone = if self.offset().is_utc() {
            "Z".to_owned()
        } else {
            self.offset().to_string()
        };
        format!(
            "{:02}:{:02}:{:02}.{:03} {}",
            self.hour(),
            self.minute(),
            self.second(),
            self.millisecond(),
            zone
        )
    }

    fn format_detailed(&self) -> String {
        // These Display implementations handle the full supported year range
        // and preserve subsecond precision without a fallible format operation.
        format!("{}T{}{}", self.date(), self.time(), self.offset())
    }
}
