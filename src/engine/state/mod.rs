mod player_data;

use std::rc::Rc;
use std::sync::mpsc::{channel,Receiver};
use super::{Action,Behavior,Map,Message,Messenger,Generator,Populator,TileType,Direction};
pub use self::player_data::PlayerData;

/// A `State` represents the current state of the game, and contains a few other important pieces
/// to run the game.
pub struct State<'a, G: Generator + 'a, P: Populator, F: Fn(Messenger) -> P + 'a> {
    /// Whether the game has been quit by the player yet
    pub quit: bool,
    map: Map,
    pd: Rc<PlayerData>,
    level: u32,
    messenger: Messenger,
    receiver: Receiver<Message>,
    generator: &'a G,
    populator: &'a F,
}

/// A `BState` is a more basic representation of a `State`, which is what gets passed to the
/// `Outputter`
#[derive(Clone)]
pub struct BState {
    pub map: Map,
    pub score: i32,
    pub money: i32,
    pub health: i32,
    pub level: u32,
    pub inventory: Vec<String>,
}

impl<'a, G: Generator, P: Populator, F: Fn(Messenger) -> P + 'a> State<'a, G, P, F> {
    /// Creates the initial `State`
    pub fn new(generator: &'a G, populator: &'a F) -> Self {
        let (sender, receiver) = channel();
        let messenger = Messenger::new(sender);
        let pd = Rc::new(PlayerData::new());
        let map = populator(messenger.clone()).populate(Map::new(1, generator), pd.clone());
        let state = Self{
            map,
            level: 0,
            pd,
            quit: false,
            messenger,
            receiver,
            generator,
            populator,
        };
        state.process_all(vec![])
    }

    /// Sets the quit field of the `State`
    pub fn quit(self) -> Self {
        Self{ quit: true, ..self }
    }

    /// Takes an `Action` and the previous `State` and produces the next `State`,
    /// kind of like a flux reducer.
    pub fn process(self, action: Action) -> Self {
        match action {
            Action::Quit    => self.quit(),
            _               => {
                let actions = self.map.process(action);
                self.process_all(actions)
            }
        }
    }

    fn process_all(mut self, behaviors: Vec<Box<Behavior>>) -> Self {
        for (index, behavior) in behaviors.into_iter().enumerate() {
            behavior.exec(index, &mut self.map);
        }
        while let Ok(message) = self.receiver.try_recv() {
            self = self.respond_to(message);
        }
        self
    }

    fn respond_to(mut self, message: Message) -> Self {
        match message {
            Message::LevelEnd => {
                self.level += 1;
                self.map = (self.populator)(self.messenger.clone()).populate(Map::new(self.level, self.generator), self.pd.clone());
            }
            Message::Die(i) => {
                self.map.tiles[i].empty();
            }
            Message::Drop(i, actor) => {
                self.map.tiles[i].fill(actor);
            }
            Message::GameOver => {
                // TODO: make a game over screen
                self = self.quit();
            },
            Message::Reveal(i) => {
                use self::TileType::*;
                // TODO: this doesn't work!
                self.flood_reveal(i);
                if self.map.tiles[i].kind() == Door {
                    let neighbours: Vec<usize> = Direction::cardinals()
                        .into_iter()
                        .filter_map(|dir| self.map.get_neighbouring_tile_index(i, dir))
                        .collect();
                    for n in neighbours.into_iter() { // any way to do this without collecting and re-iterating?
                        self.flood_reveal(n);
                    }
                }
            }
        }
        self
    }

    fn flood_reveal(&mut self, index: usize) {
        use self::TileType::*;
        if !self.map.tiles[index].foggy() { return; }
        self.map.tiles[index].reveal();
        if self.map.tiles[index].kind() == Floor || self.map.tiles[index].kind() == Hall {
            let neighbours: Vec<usize> = Direction:: variants()
                .into_iter()
                .filter_map(|dir| self.map.get_neighbouring_tile_index(index, dir))
                .filter(|n| match self.map.tiles[index].kind() {
                    Floor =>
                        self.map.tiles[*n].kind() == Floor ||
                        self.map.tiles[*n].kind() == Door ||
                        self.map.tiles[*n].kind() == Wall,
                    Hall =>
                        self.map.tiles[*n].kind() == Hall ||
                        self.map.tiles[*n].kind() == Door,
                    _ => false
                })
                .collect();
            for n in neighbours.into_iter() {
                self.flood_reveal(n);
            }
        }
    }

    /// Creates the simplified `BState` for this `State`
    pub fn simplify(&self) -> BState {
        BState{
            map: self.map.clone(),
            score: self.pd.score.get(),
            money: self.pd.money.get(),
            level: self.level,
            health: self.pd.health.get(),
            inventory: self.pd.inventory.borrow().iter().map(|i| format!("{}", i.long_name())).collect(),
        }
    }
}
