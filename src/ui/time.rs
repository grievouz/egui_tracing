use chrono::{DateTime, Local};

pub trait SpecificFormats {
    fn format_short(&self) -> String;
    fn format_detailed(&self) -> String;
}

impl SpecificFormats for DateTime<Local> {
    fn format_short(&self) -> String {
        return self.format("%H:%M:%S%.3f").to_string();
    }
    fn format_detailed(&self) -> String {
        return self.format("%Y-%m-%dT%H:%M:%S%.f%:z").to_string();
    }
}
