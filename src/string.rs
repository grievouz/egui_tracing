use unicode_segmentation::UnicodeSegmentation;

pub trait Ellipse {
    fn truncate_graphemes(&self, len: usize) -> String;
}

impl Ellipse for String {
    fn truncate_graphemes(&self, len: usize) -> String {
        if self.len() <= len {
            return self.clone();
        }

        let mut trucated = self
            .graphemes(true)
            .take(len)
            .collect::<Vec<&str>>()
            .join("");
        trucated.push_str("...");

        trucated
    }
}
