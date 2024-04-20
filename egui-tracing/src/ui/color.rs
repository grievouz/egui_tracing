use egui::Color32;

pub const TRACE_COLOR: Color32 = Color32::from_rgb(117, 80, 123);
pub const DEBUG_COLOR: Color32 = Color32::from_rgb(114, 159, 207);
pub const INFO_COLOR: Color32 = Color32::from_rgb(78, 154, 6);
pub const WARN_COLOR: Color32 = Color32::from_rgb(196, 160, 0);
pub const ERROR_COLOR: Color32 = Color32::from_rgb(204, 0, 0);

pub trait ToColor32 {
    fn to_color32(self) -> Color32;
}

impl ToColor32 for tracing::Level {
    fn to_color32(self) -> Color32 {
        match self {
            Self::TRACE => TRACE_COLOR,
            Self::DEBUG => DEBUG_COLOR,
            Self::INFO => INFO_COLOR,
            Self::WARN => WARN_COLOR,
            Self::ERROR => ERROR_COLOR,
        }
    }
}
