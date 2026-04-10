/// Contains the text labels that used in the GUI.
///
/// This is used to enable multiple language support.
pub struct TracingLabels {
    pub time: String,
    pub level: String,
    pub target: String,
    pub message: String,
    pub to_bottom: String,
    pub scroll_to_bottom: String,
    pub clear: String,
    pub clear_events: String,
    pub level_filter: String,
    pub target_filter: String,
    pub add: String,
    pub delete: String,
    pub example: String,
}

impl Default for TracingLabels {
    /// Creates a new object with the default english labels.
    fn default() -> Self {
        Self {
            time: "Time".to_string(),
            level: "Level".to_string(),
            target: "Target".to_string(),
            message: "Message".to_string(),
            to_bottom: "To Bottom".to_string(),
            scroll_to_bottom: "Scroll to Bottom".to_string(),
            clear: "Clear".to_string(),
            clear_events: "Clear Events".to_string(),
            level_filter: "Level Filter".to_string(),
            target_filter: "Target Filter".to_string(),
            add: "Add".to_string(),
            delete: "Delete".to_string(),
            example: "example".to_string(),
        }
    }
}
