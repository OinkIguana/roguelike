use engine::{Actor,Messenger,Message};

#[derive(Clone)]
pub struct Stairs {
    messenger: Messenger,
}

impl Stairs {
    pub fn new(messenger: Messenger) -> Stairs {
        Stairs{ messenger }
    }
}

impl Actor for Stairs {
    fn can_be_stepped_on(&self) -> bool { true }
    fn be_stepped_on(&mut self, _: &mut Actor) {
        self.messenger.send(Message::LevelEnd);
    }
    fn symbol(&self) -> char { '/' }
}
