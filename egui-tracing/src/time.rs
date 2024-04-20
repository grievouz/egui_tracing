use chrono::{DateTime, Local};

pub trait DateTimeFormatExt {
    fn format_short(&self) -> String;
    fn format_detailed(&self) -> String;
}

impl DateTimeFormatExt for DateTime<Local> {
    fn format_short(&self) -> String {
        self.format("%H:%M:%S%.3f").to_string()
    }
    fn format_detailed(&self) -> String {
        self.format("%Y-%m-%dT%H:%M:%S%.f%:z").to_string()
    }
}
