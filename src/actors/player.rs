use engine::{Actor,Action,Behavior,Perform,IfOpen};

#[derive(Clone)]
pub struct Player;

impl Actor for Player {
    fn react(&self, action: Action) -> Box<Behavior> {
        match action {
            Action::Move(d) => Box::new(IfOpen(d, Perform(action))),
            _               => Box::new(Perform(Action::Idle)),
        }
    }

    fn can_be_attacked(&self) -> bool { true }
    fn symbol(&self) -> char { '@' }
}
