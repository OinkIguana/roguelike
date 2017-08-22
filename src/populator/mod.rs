use engine::{Map,Populator,Messenger};

use super::actors::{Player,Bat,Gold,Stairs};

/// A Populator for an easy game
pub struct Easy {
    messenger: Messenger,
}
impl Populator for Easy {
    fn new(messenger: Messenger) -> Easy {
        Easy{ messenger }
    }

    fn populate(&self, map: Map) -> Map {
        map .fill_random_tile(Player::new())
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Bat)
            .fill_random_tile(Stairs::new(self.messenger.clone()))
    }
}
