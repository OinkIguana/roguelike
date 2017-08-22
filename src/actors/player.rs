use engine::{Actor,Action,Behavior,Perform,IfEnterable,Messenger,Message};

#[derive(Clone)]
pub struct Player {
    messenger: Messenger
}

impl Player {
    pub fn new(messenger: Messenger) -> Player {
        Player{ messenger }
    }
}

impl Actor for Player {
    fn react(&self, action: Action) -> Box<Behavior> {
        match action {
            Action::Move(d) => Box::new(IfEnterable(d, Perform(action))),
            _               => Box::new(Perform(Action::Idle)),
        }
    }

    fn can_be_attacked(&self) -> bool { true }
    fn symbol(&self) -> char { '@' }
    fn gain_money(&mut self, value: i32) {
        self.messenger.send(Message::UpdateMoney(value));
    }
}
