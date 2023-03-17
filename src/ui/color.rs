use egui::Color32;

pub(crate) const TRACE_COLOR: Color32 = Color32::from_rgb(117, 80, 123);
pub(crate) const DEBUG_COLOR: Color32 = Color32::from_rgb(114, 159, 207);
pub(crate) const INFO_COLOR: Color32 = Color32::from_rgb(78, 154, 6);
pub(crate) const WARN_COLOR: Color32 = Color32::from_rgb(196, 160, 0);
pub(crate) const ERROR_COLOR: Color32 = Color32::from_rgb(204, 0, 0);

pub trait ToColor32 {
    fn to_color32(self) -> Color32;
}

impl ToColor32 for tracing::Level {
    fn to_color32(self) -> Color32 {
        match self {
            tracing::Level::TRACE => TRACE_COLOR,
            tracing::Level::DEBUG => DEBUG_COLOR,
            tracing::Level::INFO => INFO_COLOR,
            tracing::Level::WARN => WARN_COLOR,
            tracing::Level::ERROR => ERROR_COLOR,
        }
    }
}
