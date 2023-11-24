use crate::wasm4;

pub struct Palette {
    color_1: u32,
    color_2: u32,
    color_3: u32,
    color_4: u32
}

pub const PALETTE_DEMICHROME: [u32; 4] = [
    0x211e20,
    0x555568,
    0xa0a08b,
    0xe9efec,];

pub enum Color {
    None,
    One,
    Two,
    Three,
    Four
}


pub trait Display {
    fn render(&self);
}

impl Palette {
    pub fn new(palette: [u32; 4]) -> Self {
        Self {
            color_1: palette[0],
            color_2: palette[1],
            color_3: palette[2],
            color_4: palette[3],
        }
    }

    pub fn set_colors(&self) {
        let palette_vec: [u32; 4] = [
            self.color_1,
            self.color_2,
            self.color_3,
            self.color_4,
        ];
        unsafe {
            *wasm4::PALETTE = palette_vec;
        }
    }

    pub fn use_colors(primary: Color, secondary: Option<Color>) {
        let color: u16 = match secondary {
            Some(secondary) => (primary as u16) << 4 | (secondary as u16),
            None => primary as u16
        };
        unsafe {
            *wasm4::DRAW_COLORS = color.into();
        }
    }
}
