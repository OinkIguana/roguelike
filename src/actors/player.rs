use engine::{Actor,Action,Behavior,Perform,IfEnterable,IfAttackable,IfInteractable,Messenger,Message};

#[derive(Clone)]
pub struct Player {
    health: i8,
    messenger: Messenger,
    inventory: Vec<Box<Actor>>,
}

impl Player {
    pub fn new(messenger: Messenger) -> Player {
        Player{ messenger, health: 100, inventory: vec![] }
    }
}

impl Actor for Player {
    fn react(&self, action: Action) -> Box<Behavior> {
        match action {
            Action::Move(d)     => Box::new(IfEnterable(d, Perform(action))),
            Action::Attack(d)   => Box::new(IfAttackable(d, Perform(action))),
            Action::Interact(d) => Box::new(IfInteractable(d, Perform(action))),
            _                   => Box::new(Perform(Action::Idle)),
        }
    }

    fn can_be_attacked(&self, _: &Actor) -> bool { true }
    fn be_attacked(&mut self, other: &mut Actor) {
        self.health -= other.attack_power() as i8;
        self.messenger.send(Message::SetHealth(self.health));
        if self.health <= 0 {
            self.messenger.send(Message::GameOver);
        }
    }
    fn attack_power(&self) -> u32 { 5 }

    fn symbol(&self) -> char { '@' }

    fn set_money_rel(&mut self, value: i32) {
        self.messenger.send(Message::UpdateMoney(value));
    }
    fn set_health_rel(&mut self, amount: i32) {
        self.health += amount as i8;
        if self.health > 100 { self.health = 100; }
        self.messenger.send(Message::SetHealth(self.health));
    }

    fn pick_up(&mut self, item: Box<Actor>) {
        self.inventory.push(item);
    }
}
