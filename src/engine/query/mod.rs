use super::{Tile,Map,Direction};

/// Finds the index of the first `Tile` that matches a predicate. Returns `None` if no `Tile`
/// matches.
pub struct Find<F: Fn(&Tile) -> bool>(pub F);
impl<F: Fn(&Tile) -> bool> Query for Find<F> {
    type R = usize;
    fn exec(&self, map: &Map) -> Option<Self::R> {
        map.tiles.iter().enumerate().find(|&(_, t)| self.0(&t)).map(|p| p.0)
    }
}

/// Calculates the direction from the first point to the second, both represented by their
/// tile indices.
pub struct DirectionTo(pub usize, pub usize);
impl Query for DirectionTo {
    type R = Direction;
    fn exec(&self, map: &Map) -> Option<Self::R> {
        if self.0 == self.1 {
            None
        } else {
            Some(map.get_direction(self.0, self.1))
        }
    }
}

/// Calculates the distance between two points on the `Map`, represented by their tile indices
pub struct DistanceTo(pub usize, pub usize);
impl Query for DistanceTo {
    type R = usize;
    fn exec(&self, map: &Map) -> Option<Self::R> {
        if self.0 == self.1 {
            None
        } else {
            Some(map.get_distance(self.0, self.1))
        }
    }
}

/// Executes one `Query`, then maps its value to another `Query`
pub struct Then<Ra, Rb, A, B, F>(A, F)
where   A: Query<R=Ra>,
        B: Query<R=Rb>,
        F: Fn(Ra) -> B;
impl<Ra, Rb, A, B, F> Query for Then<Ra, Rb, A, B, F>
where   A: Query<R=Ra>,
        B: Query<R=Rb>,
        F: Fn(Ra) -> B {
    type R = Rb;
    fn exec(&self, map: &Map) -> Option<Self::R> {
        self.0.exec(map).map(&self.1).and_then(|q| q.exec(map))
    }
}

/// Executes two `Query`s, returning either the results of both or neither.
impl<Ra, Rb, A, B> Query for (A, B)
where   A: Query<R=Ra>,
        B: Query<R=Rb> {
    type R = (Ra, Rb);
    fn exec(&self, map: &Map) -> Option<Self::R> {
        self.0.exec(map).and_then(|ra| self.1.exec(map).map(|rb| (ra, rb)))
    }
}

/// A `Query` provides encapsulated access to information about the game and `Map`
pub trait Query {
    type R;
    /// Executes the `Query`, returning the result if the `Query` was executed successfully
    fn exec(&self, map: &Map) -> Option<Self::R>;

    /// Chains this `Query` to another one using a `Then`
    fn then<Rb, B, F>(self, next: F) -> Then<Self::R, Rb, Self, B, F>
    where   B: Query<R=Rb>,
            F: Fn(Self::R) -> B,
            Self: Query + Sized {
        Then(self, next)
    }
}
