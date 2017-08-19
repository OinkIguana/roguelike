use engine::{Map,Populator};

use super::actors::{Player,Bat,Gold};

/// A Populator for an easy game
pub struct Easy;
impl Populator for Easy {
    fn populate(&self, map: Map) -> Map {
        map .fill_random_tile(Player::new())
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Bat)
    }
}
