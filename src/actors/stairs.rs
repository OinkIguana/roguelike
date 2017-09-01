use engine::{Actor,Messenger,Message};

/// The `Stairs` are the goal for each floor. The level ends when the `Player` steps on them.
///
/// Symbol: `/`
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
    fn can_be_stepped_on(&self, _: &Actor) -> bool { true }
    fn be_stepped_on(&mut self, _: &mut Actor) {
        self.messenger.send(Message::LevelEnd);
    }
    fn symbol(&self) -> char { '/' }
}
