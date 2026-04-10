use std::borrow::Cow;

/// Contains text labels used in the UI.
///
/// Start from [`Labels::default()`] (English) and override fields for
/// your language. Uses `Cow<'static, str>` so the defaults are zero-alloc.
///
/// ```
/// let mut labels = egui_tracing::Labels::default();
/// labels.time = "Zeit".into();
/// labels.level = "Stufe".into();
/// ```
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Labels {
    // Table header
    pub time: Cow<'static, str>,
    pub level: Cow<'static, str>,
    pub target: Cow<'static, str>,
    pub message: Cow<'static, str>,

    // Buttons
    pub clear: Cow<'static, str>,
    pub to_bottom: Cow<'static, str>,
    pub close: Cow<'static, str>,

    // Detail panel
    pub event_details: Cow<'static, str>,
    pub message_too_long: Cow<'static, str>,

    // Level filter popup
    pub level_filter: Cow<'static, str>,

    // Target filter popup
    pub target_filter: Cow<'static, str>,
    pub add: Cow<'static, str>,
    pub delete: Cow<'static, str>,
    pub target_placeholder: Cow<'static, str>,
}

impl Default for Labels {
    fn default() -> Self {
        Self {
            time: "Time".into(),
            level: "Level".into(),
            target: "Target".into(),
            message: "Message".into(),
            clear: "Clear".into(),
            to_bottom: "To Bottom".into(),
            close: "Close".into(),
            event_details: "Event Details".into(),
            message_too_long: "(message too long)".into(),
            level_filter: "Level Filter".into(),
            target_filter: "Target Filter".into(),
            add: "Add".into(),
            delete: "Delete".into(),
            target_placeholder: "example: eframe::*".into(),
        }
    }
}
