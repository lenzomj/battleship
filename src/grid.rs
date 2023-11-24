use crate::display::{Display, Palette, Color};
use crate::wasm4;

pub struct Grid {
    pub rows: u8,
    pub cols: u8,
    pub scale: u8
}

impl Grid {
    pub fn new(size: u8, scale: u8) -> Self {
        Self {
            rows: size,
            cols: size,
            scale: scale
        }
    }

    pub fn width(&self) -> u32 {
        (self.cols * self.scale) as u32
    }

    pub fn height(&self) -> u32 {
        (self.rows * self.scale) as u32
    }
}

impl Display for Grid {
    fn render(&self) {
        Palette::use_colors(Color::Two, None);
        for row in 0..=self.rows {
            let row_y: i32 = (row * self.scale) as i32;
            wasm4::hline(0, row_y, self.width());
            for col in 0..=self.cols {
                let col_x: i32 = (col * self.scale) as i32;
                wasm4::vline(col_x, 0, self.height());
            }
        }
    }
}
