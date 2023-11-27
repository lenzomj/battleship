use crate::display::{Display, Palette, Color};
use crate::wasm4;

pub struct Grid {
    pub rows: i32,
    pub cols: i32,
    pub scale: i32,
    pub hover_xy: (i32, i32),
}

const ROWS: i32 = 10;
const COLS: i32 = 10;
const SCALE: i32 = 15;

impl Grid {
    pub fn new() -> Self {
        Self {
            rows: ROWS,
            cols: COLS,
            scale: SCALE,
            hover_xy: (0, 0),
        }
    }

    pub fn hover(&mut self, mouse_x: i32, mouse_y: i32) {
        let mut hover_x = (mouse_x / self.scale) * self.scale;
        let mut hover_y = (mouse_y / self.scale) * self.scale;

        if hover_x >= self.width() {
            hover_x = self.width() - self.scale;
        }

        if hover_y >= self.height() {
            hover_y = self.height() - self.scale;
        }
        self.hover_xy = (hover_x, hover_y);
    }

    pub fn width(&self) -> i32 {
        self.cols * self.scale
    }

    pub fn height(&self) -> i32 {
        self.rows * self.scale
    }
}

impl Display for Grid {
    fn render(&self) {
        Palette::use_colors(Color::Two, None);
        for row in 0..=self.rows {
            let y: i32 = row * self.scale;
            wasm4::hline(0, y, (self.width() + 1) as u32);
            for col in 0..=self.cols {
                let x: i32 = col * self.scale;
                wasm4::vline(x, 0, (self.height() + 1) as u32);
            }
        }

        Palette::use_colors(Color::One, Some(Color::Four));
        wasm4::rect(self.hover_xy.0, self.hover_xy.1, self.scale as u32 + 1, self.scale as u32 + 1);
    }
}
