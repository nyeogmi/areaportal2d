use euclid::{Point2D, Vector2D, point2, vec2};
use moogle::IdLike;

use crate::egocentric::Egocentric;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cardinal {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Cardinal {
    pub fn right(&self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::East,
            Cardinal::East => Cardinal::South,
            Cardinal::South => Cardinal::West,
            Cardinal::West => Cardinal::North,
        }
    }

    pub fn left(&self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::West,
            Cardinal::East => Cardinal::North,
            Cardinal::South => Cardinal::East,
            Cardinal::West => Cardinal::South,
        }
    }

    pub fn reverse(&self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::South,
            Cardinal::East => Cardinal::West,
            Cardinal::South => Cardinal::North,
            Cardinal::West => Cardinal::East,
        }
    }

    pub fn rotated(&self, ego: Egocentric) -> Cardinal {
        match ego {
            Egocentric::Forward => *self,
            Egocentric::Right => self.right(),
            Egocentric::Backward => self.reverse(),
            Egocentric::Left => self.left(),
        }
    }

    pub const fn rotate_vec<U>(&self, v: Vector2D<isize, U>) -> Vector2D<isize, U> {
        match self {
            Cardinal::North => v,
            Cardinal::East => vec2(-v.y, v.x),
            Cardinal::South => vec2(-v.x, -v.y),
            Cardinal::West => vec2(v.y, -v.x),
        }
    }

    pub const fn rotate_point<U>(&self, v: Point2D<isize, U>) -> Point2D<isize, U> {
        let v: Vector2D<isize, U> = self.rotate_vec(vec2(v.x, v.y));
        return point2(v.x, v.y)
    }

    pub const fn offset<U>(&self) -> Vector2D<isize, U> {
        self.offset_by(1)
    }

    pub const fn offset_by<U>(&self, sz: isize) -> Vector2D<isize, U> {
        self.rotate_vec(vec2(0, -sz))
    }
}

impl IdLike for Cardinal {
    fn id_min_value() -> Self {
        Cardinal::North
    }

    fn id_max_value() -> Self {
        Cardinal::West
    }
}