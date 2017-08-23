use std::sync::mpsc::{channel,Receiver};
use super::{Action,Behavior,Map,Message,Messenger,Generator,Populator};
// TODO: pass these in somehow instead of directly using them in this module


/// A `State` represents the current state of the game. By serializing the state, the entire game
/// should be reproducable exactly as it was before.
pub struct State<'a, G: Generator + 'a, P: Populator, F: Fn(Messenger) -> P + 'a> { // TODO: are all the fields pub?
    /// The dungeon map
    pub map: Map,
    /// The total score accumulated by the player
    pub score: i32,
    /// The current money owned by the player
    pub money: i32,
    /// The current floor of the dungeon the player is on
    pub level: u32,
    /// Whether the game has been quit by the player
    pub quit: bool,
    /// The Messenger that this state uses to send events on
    messenger: Messenger,
    /// The Receiver that connects to the Messenger
    receiver: Receiver<Message>,
    generator: &'a G,
    populator: &'a F,
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
        }
        self
    }
}
