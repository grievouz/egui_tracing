use egui::Color32;

const MAGENTA: Color32 = Color32::from_rgb(117, 80, 123);
const BLUE: Color32 = Color32::from_rgb(114, 159, 207);
const YELLOW: Color32 = Color32::from_rgb(196, 160, 0);
const GREEN: Color32 = Color32::from_rgb(78, 154, 6);
const RED: Color32 = Color32::from_rgb(204, 0, 0);

pub trait ToColor32 {
    fn to_color32(self) -> Color32;
}

impl ToColor32 for tracing::Level {
    fn to_color32(self) -> Color32 {
        match self {
            tracing::Level::TRACE => MAGENTA,
            tracing::Level::DEBUG => BLUE,
            tracing::Level::INFO => GREEN,
            tracing::Level::WARN => YELLOW,
            tracing::Level::ERROR => RED,
        }
    }
}
