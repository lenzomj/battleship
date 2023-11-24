#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod display;
mod grid;

use wasm4::*;
use display::*;
use grid::*;

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

#[no_mangle]
fn start() {
    let palette: Palette = Palette::new(PALETTE_DEMICHROME);
    palette.set_colors();
}

#[no_mangle]
fn update() {
    //Palette::use_colors(Color::Two, None);
    //text("Hello from Rust!", 10, 10);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        Palette::use_colors(Color::Four, None);
    }

    let grid: Grid = Grid::new(10, 10);
    grid.render();

    //blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    //text("Press X to blink", 16, 90);
}
