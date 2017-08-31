use rand::{thread_rng,Rng};
use engine::{Actor,Action,Behavior,Direction,Perform,IfAttackable,IfOpen,Switch,Messenger,Message};
use super::Gold;

/// A Bat is the most simple enemy, which moves around randomly, attacking the Player if it happens
/// to be beside them.
///
/// Symbol: `B`
/// Affinity: -1
///
/// Stats:
/// *   Health - 15
/// *   Attack - 2
/// *   Defense - 0
///
/// Drops:
/// *   Gold x 1
#[derive(Clone)]
pub struct Bat {
    health: i8,
    location: usize,
    messenger: Messenger,
}
impl Bat {
    pub fn new(messenger: Messenger) -> Bat {
        Bat{ health: 15, location: 0, messenger }
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

    fn can_be_attacked(&self, other: &Actor) -> bool {
        other.affinity() >= 0
    }
    fn be_attacked(&mut self, other: &mut Actor) {
        self.health -= other.attack_power() as i8;
        if self.health <= 0 {
            self.messenger.send(Message::Die(self.get_location()));
            self.messenger.send(Message::Drop(self.get_location(), Box::new(Gold::new(1))));
        }
    }
    fn attack_power(&self) -> u32 { 2 }

    fn symbol(&self) -> char { 'B' }
    fn affinity(&self) -> i8 { -1 }

    fn get_location(&self) -> usize { return self.location; }
    fn set_location(&mut self, location: usize) { self.location = location; }
}
