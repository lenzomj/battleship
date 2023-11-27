use std::sync::Mutex;
use fastrand::Rng;
use lazy_static::lazy_static;

const SEED: u64 = 737275;

lazy_static! {
    pub static ref RNG: Mutex<Rng> = Mutex::new(Rng::with_seed(SEED));
}
