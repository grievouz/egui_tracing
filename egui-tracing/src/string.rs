use unicode_segmentation::UnicodeSegmentation;

pub trait Ellipse {
    fn truncate_graphemes(&self, len: usize) -> String;
}

impl Ellipse for String {
    fn truncate_graphemes(&self, len: usize) -> String {
        if self.len() <= len {
            return self.clone();
        }

        let mut truncated: String = self.graphemes(true).take(len).collect();
        truncated.push_str("...");

        truncated
    }
}
