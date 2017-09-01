use std::rc::Rc;
use engine::{PlayerData,Map,Populator,Messenger,Message};

use super::actors::*;

/// A Populator for an very basic, easy game
pub struct Easy {
    messenger: Messenger,
}
impl Populator for Easy {
    fn new(messenger: Messenger) -> Easy {
        Easy{ messenger }
    }

    fn populate(&self, map: Map, pd: Rc<PlayerData>) -> Map {
        let player_tile = map.get_random_open_tile().expect("There must be at least one tile available still...");
        self.messenger.send(Message::Reveal(player_tile));
        map .fill_tile(player_tile, Player::new(self.messenger.clone(), pd))
            .fill_random_tile(Stairs::new(self.messenger.clone()))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Gold::new(5))
            .fill_random_tile(Bat::new(self.messenger.clone()))
            .fill_random_tile(Bat::new(self.messenger.clone()))
            .fill_random_tile(Bat::new(self.messenger.clone()))
            .fill_random_tile(Goblin::new(self.messenger.clone()))
            .fill_random_tile(Fountain::new())
            .fill_random_tile(Potion)
            .fill_random_tile(Potion)
            .fill_random_tile(Chest::new(Gold::new(10)))
            .fill_random_tile(Key)
    }
}
