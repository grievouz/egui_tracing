use std::ops::RangeInclusive;

use egui::{Ui, Vec2};

#[derive(Default, Debug)]
pub struct CommonProps {
    enabled: Option<bool>,
    height: Option<f32>,
    height_range: Option<RangeInclusive<f32>>,
    max_height: Option<f32>,
    max_size: Option<Vec2>,
    max_width: Option<f32>,
    min_width: Option<f32>,
    min_height: Option<f32>,
    min_size: Option<Vec2>,
    row_height: Option<f32>,
    visible: Option<bool>,
    width: Option<f32>,
    width_range: Option<RangeInclusive<f32>>,
}

impl CommonProps {
    pub fn min_width(mut self, v: f32) -> Self {
        self.min_width = Some(v);
        self
    }
}

pub fn set_common_props(ui: &mut Ui, c: &Option<CommonProps>) {
    if let Some(c) = c {
        if let Some(v) = c.enabled {
            ui.set_enabled(v);
        }
        if let Some(v) = c.height {
            ui.set_height(v);
        }
        if let Some(v) = c.height_range.clone() {
            ui.set_height_range(v);
        }
        if let Some(v) = c.max_height {
            ui.set_max_height(v);
        }
        if let Some(v) = c.max_size {
            ui.set_max_size(v);
        }
        if let Some(v) = c.max_width {
            ui.set_max_width(v);
        }
        if let Some(v) = c.min_width {
            ui.set_min_width(v);
        }
        if let Some(v) = c.min_height {
            ui.set_min_height(v);
        }
        if let Some(v) = c.min_size {
            ui.set_min_size(v);
        }
        if let Some(v) = c.row_height {
            ui.set_row_height(v);
        }
        if let Some(v) = c.visible {
            ui.set_visible(v);
        }
        if let Some(v) = c.width {
            ui.set_width(v);
        }
        if let Some(v) = c.width_range.clone() {
            ui.set_width_range(v);
        }
    }
}
