#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

mod board;
mod display;
mod game;
mod grid;
mod ship;
mod rand;

use lazy_static::lazy_static;
use std::sync::Mutex;

use wasm4::*;
use board::*;
use game::*;
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

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    Palette::set(PALETTE_DEMICHROME);
}

#[no_mangle]
fn update() {
    //Palette::use_colors(Color::Two, None);
    //text("Hello from Rust!", 10, 10);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        Palette::use_colors(Color::Four, None);
        GAME.lock().unwrap().advance();
    }

    let mut grid: Grid = Grid::new();
    let mouse_x = unsafe { *MOUSE_X };
    let mouse_y = unsafe { *MOUSE_Y };
    grid.hover(i32::from(mouse_x), i32::from(mouse_y));

    grid.render();

    //blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    //text("Press X to blink", 16, 90);
}
