use engine::{Map,Populator,Messenger};

use super::actors::*;

/// A Populator for an very basic, easy game
pub struct Easy {
    messenger: Messenger,
}
impl Populator for Easy {
    fn new(messenger: Messenger) -> Easy {
        Easy{ messenger }
    }

    fn populate(&self, map: Map) -> Map {
        map .fill_random_tile(Player::new(self.messenger.clone()))
            .fill_random_tile(Stairs::new(self.messenger.clone()))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Bat::new(self.messenger.clone()))
            .fill_random_tile(Bat::new(self.messenger.clone()))
            .fill_random_tile(Bat::new(self.messenger.clone()))
            .fill_random_tile(Fountain::new())
    }
}
