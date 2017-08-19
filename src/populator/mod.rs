use engine::{Map,Populator};

use super::actors::{Player,Bat};

/// A Populator for an easy game
pub struct Easy;
impl Populator for Easy {
    fn populate(&self, map: Map) -> Map {
        map .fill_random_tile(Player)
            .fill_random_tile(Bat)
    }
}
