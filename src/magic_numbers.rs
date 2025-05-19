use bevy::{color::Color, ui::Val};

pub static CELL_ROWS: i8 = 16;
pub static CELL_COLS: i8 = 13;
pub static CELL_SIZE_PX: Val = Val::Px(32.);
pub static BORDER: Val = Val::Px(1.);
pub static GREY_COLOR: Color = Color::linear_rgb(0.2, 0.2, 0.2);
