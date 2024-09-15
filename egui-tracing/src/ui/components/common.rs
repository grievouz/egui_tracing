use std::ops::RangeInclusive;
use egui::{Ui, Vec2};

#[derive(Default, Debug, Clone)]
pub struct CommonProps {
    enabled: Option<bool>,
    visible: Option<bool>,
    height: Option<f32>,
    width: Option<f32>,
    min_size: Option<Vec2>,
    max_size: Option<Vec2>,
    height_range: Option<RangeInclusive<f32>>,
    width_range: Option<RangeInclusive<f32>>,
    min_width: Option<f32>,
    max_width: Option<f32>,
    min_height: Option<f32>,
    max_height: Option<f32>,
    row_height: Option<f32>,
}

#[allow(dead_code)]
impl CommonProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, v: bool) -> Self {
        self.enabled = Some(v);
        self
    }

    pub fn visible(mut self, v: bool) -> Self {
        self.visible = Some(v);
        self
    }

    pub fn height(mut self, v: f32) -> Self {
        self.height = Some(v);
        self
    }

    pub fn width(mut self, v: f32) -> Self {
        self.width = Some(v);
        self
    }

    pub fn min_size(mut self, v: Vec2) -> Self {
        self.min_size = Some(v);
        self
    }

    pub fn max_size(mut self, v: Vec2) -> Self {
        self.max_size = Some(v);
        self
    }

    pub fn height_range(mut self, v: RangeInclusive<f32>) -> Self {
        self.height_range = Some(v);
        self
    }

    pub fn width_range(mut self, v: RangeInclusive<f32>) -> Self {
        self.width_range = Some(v);
        self
    }

    pub fn min_width(mut self, v: f32) -> Self {
        self.min_width = Some(v);
        self
    }

    pub fn max_width(mut self, v: f32) -> Self {
        self.max_width = Some(v);
        self
    }

    pub fn min_height(mut self, v: f32) -> Self {
        self.min_height = Some(v);
        self
    }

    pub fn max_height(mut self, v: f32) -> Self {
        self.max_height = Some(v);
        self
    }

    pub fn row_height(mut self, v: f32) -> Self {
        self.row_height = Some(v);
        self
    }
}

pub fn set_common_props(ui: &mut Ui, props: &Option<CommonProps>) {
    let Some(props) = props else { return };

    if let Some(false) = props.enabled {
        ui.disable();
    }

    if let Some(false) = props.visible {
        ui.set_invisible();
    }

    macro_rules! set_prop {
        ($prop:ident, $method:ident) => {
            if let Some(value) = props.$prop {
                ui.$method(value);
            }
        };
        ($prop:ident, $method:ident, $transform:expr) => {
            if let Some(value) = &props.$prop {
                ui.$method($transform(value));
            }
        };
    }

    set_prop!(height, set_height);
    set_prop!(width, set_width);
    set_prop!(min_size, set_min_size);
    set_prop!(max_size, set_max_size);
    set_prop!(height_range, set_height_range, Clone::clone);
    set_prop!(width_range, set_width_range, Clone::clone);
    set_prop!(min_width, set_min_width);
    set_prop!(max_width, set_max_width);
    set_prop!(min_height, set_min_height);
    set_prop!(max_height, set_max_height);
    set_prop!(row_height, set_row_height);
}