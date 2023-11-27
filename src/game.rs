use crate::wasm4;
use crate::board::Board;

pub struct Game {
    state: GameState,
    player_1: Board,
    player_2: Board,
}

pub enum GameState {
    Setup,
    P1_Fire,
    P1_Check,
    P2_Fire,
    P2_Check,
    Score,
}

impl GameState {
    pub fn new() -> Self {
        Self::Setup
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Setup => Self::P1_Fire,
            Self::P1_Fire => Self::P1_Check,
            Self::P1_Check => Self::P2_Fire,
            Self::P2_Fire => Self::P2_Check,
            Self::P2_Check => Self::P1_Fire,
            Self::Score => Self::Setup,
        }
    }

    pub fn score(&self) -> Self {
        Self::Score
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::new(),
            player_1: Board::new(),
            player_2: Board::new(),
        }
    }

    pub fn advance(&mut self) {
        self.state = self.state.next();
    }

}
