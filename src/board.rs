use crate::ship::*;
use crate::rand::RNG;

const ROWS: usize = 10;
const COLS: usize = 10;
const SIZE: usize = 12;
const SHIPS: usize = 5;

pub struct Board {
    rows: usize,
    cols: usize,
    size: usize,
    track: Vec<(i32, i32)>,
    ships: [Ship; SHIPS],
}

impl Board {
    pub fn new() -> Self {
        Self {
            rows: ROWS,
            cols: COLS,
            size: SIZE,
            track: Vec::new(),
            ships: [
                Ship::new("CV", HULL_CLASS_5),
                Ship::new("BB", HULL_CLASS_4),
                Ship::new("DD", HULL_CLASS_3),
                Ship::new("SS", HULL_CLASS_3),
                Ship::new("PB", HULL_CLASS_2),
            ]
        }
    }

    pub fn place_ships(&mut self) {

        // Model the board as a grid
        let mut grid: Vec<Vec<char>> = vec![vec![' '; COLS]; ROWS];

        fn place_ship(grid: &mut Vec<Vec<char>>, ship: &mut Ship) -> bool {
            let row:usize = RNG.lock().unwrap().usize(0..=ROWS);
            let col:usize = RNG.lock().unwrap().usize(0..=COLS);
            match ship.get_heading() {
                Heading::EastWest if col + ship.length() <= COLS => {
                    for c in 0..ship.length() {
                        if grid[row][col + c] == 'X' {
                            return false; // Collision
                        }
                    }
                    for c in 0..ship.length() {
                        grid[row][col + c] = 'X';
                        ship.locate((row, col + c));
                    }
                    true
                },
                Heading::NorthSouth if row + ship.length() <= ROWS => {
                    for r in 0..ship.length() {
                        if grid[row + r][col] == 'X' {
                            return false; // Collision
                        }
                    }
                    for r in 0..ship.length() {
                        grid[row + r][col] = 'X';
                        ship.locate((row + r, col));
                    }
                    true
                },
                _ => false // Out of bounds
            }
        }

        for ship in self.ships.iter_mut() {
            // Randomize ship direction
            if RNG.lock().unwrap().bool() {
                ship.set_heading(Heading::NorthSouth)
            }

            // Attempt to place ship
            loop {
                if place_ship(&mut grid, ship) {
                    break;
                }
            }
        }
    }
}
