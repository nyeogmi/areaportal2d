use std::isize;

use euclid::{Point2D, Rect, Size2D, Vector2D, point2};
use moogle::Id;

use crate::{Cardinal, egocentric::Egocentric, universe::RTRoom};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct GlobalView {
    pub r: Id<RTRoom>,
    pub x: Point2D<isize, UniverseSpace>,
    pub c: Cardinal,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct GlobalPoint {
    pub r: Id<RTRoom>,
    pub x: Point2D<isize, UniverseSpace>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UniverseSpace;

pub struct EgoSpace;

pub type EgoPoint = Point2D<isize, EgoSpace>;
pub type EgoSize = Size2D<isize, EgoSpace>;
pub type EgoVec = Vector2D<isize, EgoSpace>;
pub type EgoRect = Rect<isize, EgoSpace>;

impl moogle::IdLike for GlobalView {
    fn id_min_value() -> Self {
        GlobalView {
            r: Id::id_min_value(),
            x: point2(isize::id_min_value(), isize::id_min_value()),
            c: Cardinal::id_min_value(),
        }
    }

    fn id_max_value() -> Self {
        GlobalView {
            r: Id::id_max_value(),
            x: point2(isize::id_max_value(), isize::id_max_value()),
            c: Cardinal::id_max_value(),
        }
    }
}

impl PartialOrd for GlobalView {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some( (self.r, self.x.x, self.x.y, self.c).cmp(&(other.r, other.x.y, other.x.y, other.c)) )
    }
}

impl Ord for GlobalView {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       (self.r, self.x.x, self.x.y, self.c).cmp(&(other.r, other.x.y, other.x.y, other.c))
    }
}

impl moogle::IdLike for GlobalPoint {
    fn id_min_value() -> Self {
        GlobalPoint {
            r: Id::id_min_value(),
            x: point2(isize::id_min_value(), isize::id_min_value()),
        }
    }

    fn id_max_value() -> Self {
        GlobalPoint {
            r: Id::id_max_value(),
            x: point2(isize::id_max_value(), isize::id_max_value()),
        }
    }
}

impl PartialOrd for GlobalPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some( (self.r, self.x.x, self.x.y).cmp(&(other.r, other.x.y, other.x.y)) )
    }
}

impl Ord for GlobalPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       (self.r, self.x.x, self.x.y).cmp(&(other.r, other.x.y, other.x.y))
    }
}

impl GlobalView {
    pub fn rotated(&self, ego: Egocentric) -> GlobalView {
        GlobalView {
            r: self.r,
            x: self.x,
            c: self.c.rotated(ego),
        }
    }
}