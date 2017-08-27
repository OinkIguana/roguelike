use std::sync::mpsc::{channel,Receiver};
use super::{Action,Behavior,Map,Message,Messenger,Generator,Populator};

/// A `State` represents the current state of the game. By serializing the state, the entire game
/// should be reproducable exactly as it was before.
pub struct State<'a, G: Generator + 'a, P: Populator, F: Fn(Messenger) -> P + 'a> { // TODO: are all the fields pub?
    /// Whether the game has been quit by the player
    pub quit: bool,
    /// The dungeon map
    map: Map,
    /// The total score accumulated by the player
    score: i32,
    /// The current money owned by the player
    money: i32,
    /// The current floor of the dungeon the player is on
    level: u32,
    /// The health of the player, to display on the HUD
    health: i32,
    messenger: Messenger,
    receiver: Receiver<Message>,
    generator: &'a G,
    populator: &'a F,
}

/// A `BState` is a more basic representation of a state, which is what gets passed to the
/// Outputter
#[derive(Clone)]
pub struct BState {
    pub map: Map,
    pub score: i32,
    pub money: i32,
    pub health: i32,
    pub level: u32,
}

impl<'a, G: Generator, P: Populator, F: Fn(Messenger) -> P + 'a> State<'a, G, P, F> {
    /// Creates the initial state
    pub fn new(generator: &'a G, populator: &'a F) -> Self {
        let (sender, receiver) = channel();
        let messenger = Messenger::new(sender);
        State{
            map: Map::new(1, generator).populate(&populator(messenger.clone())),
            score: 0,
            money: 0,
            level: 1,
            health: 100,
            quit: false,
            messenger,
            receiver,
            generator,
            populator,
        }
    }

    /// Sets the quit field of the State
    pub fn quit(mut self) -> Self {
        self.quit = true;
        self
    }

    /// Takes an Action and the previous state and produces the next state
    /// kind of like a flux reducer...
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
        loop {
            if let Ok(message) = self.receiver.try_recv() {
                self = self.respond_to(message);
            } else {
                break;
            }
        }
        self
    }

    fn respond_to(mut self, message: Message) -> Self {
        match message {
            Message::LevelEnd => {
                self.level += 1;
                self.map = Map::new(self.level, self.generator).populate(&(self.populator)(self.messenger.clone()));
            }
            Message::UpdateMoney(qty) => {
                self.money += qty;
            }
            Message::Die(index) => {
                self.map.tiles[index].empty();
            }
            Message::Drop(index, actor) => {
                self.map.tiles[index].fill(actor);
            }
            Message::SetHealth(health) => {
                self.health = health;
            }
            Message::GameOver => {
                // TODO: make a game over screen
                self = self.quit();
            }
        }
        self
    }

    pub fn simplify(&self) -> BState {
        BState{
            map: self.map.clone(),
            score: self.score,
            money: self.money,
            level: self.level,
            health: self.health,
        }
    }
}
