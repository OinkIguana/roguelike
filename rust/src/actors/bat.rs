use rand::{thread_rng,Rng};
use engine::{Actor,Action,Behavior,Direction,start,perform};

#[derive(Clone)]
pub struct Bat;

impl Actor for Bat {
    fn react(&self, _: Action) -> Box<Behavior> {
        let mut dirs = Direction::cardinals();
        thread_rng().shuffle(&mut dirs);
        start().switch(
            dirs.into_iter()
                .map(|d| start().if_attackable(d, *perform(Action::Attack(d))))
        )
    }
    fn can_be_attacked(&self) -> bool { true }
    fn symbol(&self) -> char { 'B' }
}
