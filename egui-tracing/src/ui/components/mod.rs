use egui::Ui;

pub mod common;
pub mod constants;
pub mod level_menu_button;
pub mod table;
pub mod table_cell;
pub mod table_header;
pub mod target_menu_button;
pub mod target_menu_item;

pub trait ChildFn: FnMut(&mut Ui) {}

impl<U> ChildFn for U where U: FnMut(&mut Ui) {}