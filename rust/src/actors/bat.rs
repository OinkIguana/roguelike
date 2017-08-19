use rand::{thread_rng,Rng};
use engine::{Actor,Action,Behavior,Direction,start};

#[derive(Clone)]
pub struct Bat;

impl Actor for Bat {
    fn react(&self, _: Action) -> Box<Behavior> {
        let mut dirs = Direction::cardinals();
        thread_rng().shuffle(&mut dirs);
        start()
    }
    fn can_be_attacked(&self) -> bool { true }
    fn symbol(&self) -> char { 'B' }
}
