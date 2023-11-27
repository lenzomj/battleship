use crate::wasm4;

pub trait Display {
    fn render(&self);
}

pub struct Palette;

pub const PALETTE_DEMICHROME: [u32; 4] = [
    0x211e20,
    0x555568,
    0xa0a08b,
    0xe9efec,];

pub enum Color {
    None = 0x00,
    One = 0x01,
    Two = 0x02,
    Three = 0x03,
    Four = 0x04
}

impl Palette {
    pub fn set(palette: [u32; 4]) {
        unsafe {
            *wasm4::PALETTE = palette;
        }
    }

    pub fn use_colors(primary: Color, secondary: Option<Color>) {
        let color: u16 = match secondary {
            Some(secondary) => (secondary as u16) << 4 | (primary as u16),
            None => primary as u16
        };
        unsafe {
            *wasm4::DRAW_COLORS = color.into();
        }
    }
}
