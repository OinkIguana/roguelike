use engine::{Map,Populator};

use super::actors::Player;

/// A Populator for an easy game
pub struct Easy;
impl Populator for Easy {
    fn populate(&self, map: Map) -> Map {
        let tile = map.get_random_open_tile();
        map.fill_tile(tile.expect("There must be at least one open tile..."), Player{})
    }
}
