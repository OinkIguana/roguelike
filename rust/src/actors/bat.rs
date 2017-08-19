use rand::{thread_rng,Rng};
use engine::{Actor,Action,Behavior,Direction,Perform,IfAttackable,IfOpen,Switch};

#[derive(Clone)]
pub struct Bat;

impl Actor for Bat {
    fn react(&self, _: Action) -> Box<Behavior> {
        let mut dirs = Direction::cardinals();
        thread_rng().shuffle(&mut dirs);
        let attacks: Vec<IfAttackable<Perform>> = dirs.iter().cloned().map(|d| IfAttackable(d, Perform(Action::Attack(d)))).collect();
        let moves: Vec<IfOpen<Perform>> = dirs.iter().cloned().map(|d| IfOpen(d, Perform(Action::Move(d)))).collect();
        Box::new(Switch(attacks).or_else(Switch(moves)))
    }
    fn can_be_attacked(&self) -> bool { true }
    fn symbol(&self) -> char { 'B' }
}
