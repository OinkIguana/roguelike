use std::rc::Rc;
use std::cmp::max;
use engine::*;

/// The `Player` is the character which is controlled by the player.
#[derive(Clone)]
pub struct Player {
    pd: Rc<PlayerData>,
    messenger: Messenger,
    location: usize,
}

impl Player {
    pub fn new(messenger: Messenger, pd: Rc<PlayerData>) -> Player {
        Player{ messenger, pd, location: 0, }
    }
    pub fn id() -> &'static str {
        "Player"
    }
}

impl Actor for Player {
    fn react(&self, action: Action) -> Box<Behavior> {
        match action {
            Action::Move(d)     => Box::new(IfEnterable(d, Perform(action))),
            Action::Attack(d)   => Box::new(IfAttackable(d, Perform(action))),
            Action::Interact(d) => Box::new(IfInteractable(d, Perform(action))),
            action              => Box::new(Perform(action)),
        }
    }

    fn on_move(&mut self) {
        self.messenger.send(Message::Reveal(self.get_location()));
    }

    fn can_be_attacked(&self, _: &Actor) -> bool { true }
    fn be_attacked(&mut self, other: &mut Actor) {
        self.pd.health.set(self.pd.health.get() - other.attack_power() as i32);
        if self.pd.health.get() <= 0 {
            self.messenger.send(Message::GameOver);
        }
    }
    fn attack_power(&self) -> u32 { 5 }

    fn symbol(&self) -> char { '@' }

    fn take_money(&mut self, amount: i32) -> i32 {
        let money = max(0, self.pd.money.get() - amount);
        let taken = self.pd.money.get() - money;
        self.pd.money.set(money);
        taken
    }

    fn set_money_rel(&mut self, value: i32) {
        self.pd.money.set(self.pd.money.get() + value);
    }
    fn set_health_rel(&mut self, amount: i32) {
        self.pd.health.set(self.pd.health.get() + amount);
        if self.pd.health.get() > 100 { self.pd.health.set(100); }
    }

    fn pick_up(&mut self, item: Box<Actor>) {
        self.pd.inventory.borrow_mut().push(item);
    }
    fn get_item(&mut self, index: usize) -> Option<Box<Actor>> {
        if index < self.pd.inventory.borrow().len() {
            Some(self.pd.inventory.borrow_mut().remove(index))
        } else {
            None
        }
    }
    fn find_item(&self, name: &str) -> Option<usize> {
        self.pd.inventory.borrow().iter().position(|i| i.long_name() == name)
    }

    fn get_location(&self) -> usize {
        self.location
    }
    fn set_location(&mut self, loc: usize) {
        self.location = loc;
    }

    fn long_name(&self) -> &str {
        Player::id()
    }
}
