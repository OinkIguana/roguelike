use rand::{thread_rng,Rng};
use engine::{Actor,Action,Behavior,Direction,Perform,IfAttackable,IfOpen,Switch,Messenger,Message};

#[derive(Clone)]
pub struct Bat {
    health: i8,
    messenger: Messenger,
}
impl Bat {
    pub fn new(messenger: Messenger) -> Bat {
        Bat{ health: 15, messenger }
    }
}

impl Actor for Bat {
    fn react(&self, _: Action) -> Box<Behavior> {
        let mut dirs = Direction::cardinals();
        thread_rng().shuffle(&mut dirs);
        let attacks: Vec<IfAttackable<Perform>> = dirs.iter().cloned().map(|d| IfAttackable(d, Perform(Action::Attack(d)))).collect();
        let moves: Vec<IfOpen<Perform>> = dirs.iter().cloned().map(|d| IfOpen(d, Perform(Action::Move(d)))).collect();
        Box::new(Switch(attacks).or_else(Switch(moves)))
    }
    fn can_be_attacked(&self, _: &Actor) -> bool { true }
    fn be_attacked(&mut self, other: &mut Actor) {
        self.health -= other.calculate_attack_power() as i8;
        if self.health <= 0 {
            self.messenger.send(Message::Die);
        }
    }
    fn calculate_attack_power(&self) -> u32 { 2 }
    fn symbol(&self) -> char { 'B' }
}
