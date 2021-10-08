use std::isize;

use euclid::{Point2D, Rect, Size2D, Vector2D, point2};
use moogle::{IdLike};

use crate::{Cardinal, egocentric::Egocentric};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GlobalView<R: IdLike> {
    pub r: R,
    pub x: Point2D<isize, GlobalSpace>,
    pub c: Cardinal,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct GlobalPoint<R: IdLike> {
    pub r: R,
    pub x: Point2D<isize, GlobalSpace>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalSpace;

pub struct EgoSpace;

pub type EgoPoint = Point2D<isize, EgoSpace>;
pub type EgoSize = Size2D<isize, EgoSpace>;
pub type EgoVec = Vector2D<isize, EgoSpace>;
pub type EgoRect = Rect<isize, EgoSpace>;

impl<R: IdLike> IdLike for GlobalView<R> {
    fn id_min_value() -> Self {
        GlobalView {
            r: R::id_min_value(),
            x: point2(isize::id_min_value(), isize::id_min_value()),
            c: Cardinal::id_min_value(),
        }
    }

    fn id_max_value() -> Self {
        GlobalView {
            r: R::id_max_value(),
            x: point2(isize::id_max_value(), isize::id_max_value()),
            c: Cardinal::id_max_value(),
        }
    }
}

impl<R: IdLike> PartialOrd for GlobalView<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.r, self.x.x, self.x.y, self.c).partial_cmp(&(other.r, other.x.x, other.x.y, other.c)) 
    }
}

impl<R: IdLike> Ord for GlobalView<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       (self.r, self.x.x, self.x.y, self.c).cmp(&(other.r, other.x.x, other.x.y, other.c))
    }
}

impl<R: IdLike> IdLike for GlobalPoint<R> {
    fn id_min_value() -> Self {
        GlobalPoint {
            r: R::id_min_value(),
            x: point2(isize::id_min_value(), isize::id_min_value()),
        }
    }

    fn id_max_value() -> Self {
        GlobalPoint {
            r: R::id_max_value(),
            x: point2(isize::id_max_value(), isize::id_max_value()),
        }
    }
}

impl<R: IdLike> PartialOrd for GlobalPoint<R> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some( (self.r, self.x.x, self.x.y).cmp(&(other.r, other.x.x, other.x.y)) )
    }
}

impl<R: IdLike> Ord for GlobalPoint<R> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
       (self.r, self.x.x, self.x.y).cmp(&(other.r, other.x.x, other.x.y))
    }
}

impl<R: IdLike> GlobalView<R> {
    pub fn rotated(&self, ego: Egocentric) -> GlobalView<R> {
        GlobalView {
            r: self.r,
            x: self.x,
            c: self.c.rotated(ego),
        }
    }
}

impl<R: IdLike> GlobalView<R> {
    pub fn point(&self) -> GlobalPoint<R> {
        return GlobalPoint { 
            r: self.r,
            x: self.x,
        }
    }
}

impl<R: IdLike> GlobalPoint<R> {
    pub fn facing(&self, c: Cardinal) -> GlobalView<R> {
        return GlobalView { 
            r: self.r,
            x: self.x,
            c,
        }
    }
}