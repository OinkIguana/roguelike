use std::sync::mpsc::Sender;
use super::Actor;

/// `Message`s are used to notify the game that certain actions should be taken at this time.
#[derive(Clone)]
pub enum Message {
    /// The level has ended
    LevelEnd,
    /// The game has ended (in defeat)
    GameOver,
    /// An `Actor` has died and should be removed from the game
    Die(usize),
    /// A new `Actor` should be created at the given location
    Drop(usize, Box<Actor>),
    /// The `Tile` at the given location should be revealed (in foggy mode)
    Reveal(usize),
}

/// A `Messenger` is used to send `Message`s to the game engine.
#[derive(Clone)]
pub struct Messenger {
    sender: Sender<Message>,
}

impl Messenger {
    /// Creates a new `Messenger`
    pub fn new(sender: Sender<Message>) -> Messenger {
        Messenger{ sender }
    }

    /// Sends a `Message` along the `Messenger`
    pub fn send(&self, message: Message) {
        self.sender.send(message).unwrap();
    }
}
