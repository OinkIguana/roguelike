use engine::{Actor,Action,Behavior,Perform,IfEnterable};

#[derive(Clone)]
pub struct Player {
    pub money: i32,
}

impl Player {
    pub fn new() -> Player {
        Player{ money: 0 }
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
        self.money += value;
    }
    fn money(&self) -> i32 { self.money }
}
