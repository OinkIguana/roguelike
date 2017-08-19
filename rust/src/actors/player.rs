use engine::{Actor,Action,Behavior,perform};

#[derive(Clone)]
pub struct Player;

impl Actor for Player {
    fn react(&self, action: Action) -> Box<Behavior> { perform(action) }
    fn can_be_attacked(&self) -> bool { true }
    fn symbol(&self) -> char { '@' }
}
