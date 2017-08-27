use std::sync::mpsc::Sender;
use super::Actor;

#[derive(Clone)]
pub enum Message {
    LevelEnd,
    GameOver,
    Die(usize),
    Drop(usize, Box<Actor>),
    SetHealth(i32),
    UpdateMoney(i32),
    GetItem(String),
    RemoveItem(usize),
}

#[derive(Clone)]
pub struct Messenger {
    sender: Sender<Message>,
}

impl Messenger {
    pub fn new(sender: Sender<Message>) -> Messenger {
        Messenger{ sender }
    }

    pub fn send(&self, message: Message) {
        self.sender.send(message).unwrap();
    }
}
