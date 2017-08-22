use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;

#[derive(Clone,Copy)]
pub enum Message {
    LevelEnd,
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
