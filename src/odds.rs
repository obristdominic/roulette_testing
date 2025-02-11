extern crate rand;
use rand::Rng;

fn random_number(range: u16) -> u16 {
    rand::thread_rng().random_range(1..range)
}

pub fn win_or_loss(odds_permille: u16) -> bool {
  random_number(1000) <= odds_permille
}
