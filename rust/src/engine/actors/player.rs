use super::Actor;
use inputter::Action;

#[derive(Clone)]
pub struct Player;

impl Actor for Player {
    fn react(&self, action: Action) -> Action {
        action
    }
    fn can_be_attacked(&self) -> bool { true }
    fn symbol(&self) -> char { '@' }
}
