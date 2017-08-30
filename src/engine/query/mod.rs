use super::{Tile,Map,Direction};

pub struct Find<F: Fn(&Tile) -> bool>(pub F);
impl<F: Fn(&Tile) -> bool> Query for Find<F> {
    type R = usize;
    fn exec(&self, map: &Map) -> Option<usize> {
        map.tiles.iter().enumerate().find(|&(_, t)| self.0(&t)).map(|p| p.0)
    }
}

pub struct DirectionTo(pub usize, pub usize);
impl Query for DirectionTo {
    type R = Direction;
    fn exec(&self, map: &Map) -> Option<Direction> {
        if self.0 == self.1 {
            None
        } else {
            Some(map.get_direction(self.0, self.1))
        }
    }
}

pub struct Then<Ra, Rb, A, B, F>(A, F)
where   A: Query<R=Ra>,
        B: Query<R=Rb>,
        F: Fn(Ra) -> B;

impl<Ra, Rb, A, B, F> Query for Then<Ra, Rb, A, B, F>
where   A: Query<R=Ra>,
        B: Query<R=Rb>,
        F: Fn(Ra) -> B {
    type R = Rb;
    fn exec(&self, map: &Map) -> Option<Rb> {
        self.0.exec(map).map(&self.1).and_then(|q| q.exec(map))
    }
}

pub trait Query {
    type R;
    fn exec(&self, map: &Map) -> Option<Self::R>;

    fn then<Rb, B, F>(self, next: F) -> Then<Self::R, Rb, Self, B, F>
    where   B: Query<R=Rb>,
            F: Fn(Self::R) -> B,
            Self: Query + Sized, {
        Then(self, next)
    }
}
