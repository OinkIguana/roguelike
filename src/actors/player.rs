use engine::{Actor,Action,Behavior,Perform,IfEnterable,IfAttackable,Messenger,Message};

#[derive(Clone)]
pub struct Player {
    health: i8,
    messenger: Messenger
}

impl Player {
    pub fn new(messenger: Messenger) -> Player {
        Player{ messenger, health: 100 }
    }
}

impl Actor for Player {
    fn react(&self, action: Action) -> Box<Behavior> {
        match action {
            Action::Move(d)     => Box::new(IfEnterable(d, Perform(action))),
            Action::Attack(d)   => Box::new(IfAttackable(d, Perform(action))),
            _                   => Box::new(Perform(Action::Idle)),
        }
    }

    fn can_be_attacked(&self, _: &Actor) -> bool { true }
    fn be_attacked(&mut self, other: &mut Actor) {
        self.health -= other.calculate_attack_power() as i8;
        self.messenger.send(Message::SetHealth(self.health));
        if self.health <= 0 {
            self.messenger.send(Message::GameOver);
        }
    }
    fn calculate_attack_power(&self) -> u32 { 5 }

    fn symbol(&self) -> char { '@' }
    fn gain_money(&mut self, value: i32) {
        self.messenger.send(Message::UpdateMoney(value));
    }
}
