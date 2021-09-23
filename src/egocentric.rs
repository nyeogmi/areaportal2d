use euclid::{Point2D, Vector2D, point2, vec2};
use moogle::IdLike;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Egocentric {
    Forward = 0,
    Right = 1,
    Backward = 2,
    Left = 3,
}

impl Egocentric {
    pub fn right(&self) -> Egocentric {
        match self {
            Egocentric::Forward => Egocentric::Right,
            Egocentric::Right => Egocentric::Backward,
            Egocentric::Backward => Egocentric::Left,
            Egocentric::Left => Egocentric::Forward,
        }
    }

    pub fn left(&self) -> Egocentric {
        match self {
            Egocentric::Forward => Egocentric::Left,
            Egocentric::Right => Egocentric::Forward,
            Egocentric::Backward => Egocentric::Right,
            Egocentric::Left => Egocentric::Backward,
        }
    }

    pub fn reverse(&self) -> Egocentric {
        match self {
            Egocentric::Forward => Egocentric::Backward,
            Egocentric::Right => Egocentric::Left,
            Egocentric::Backward => Egocentric::Forward,
            Egocentric::Left => Egocentric::Right,
        }
    }

    pub fn rotated(&self, ego: Egocentric) -> Egocentric {
        match ego {
            Egocentric::Forward => *self,
            Egocentric::Right => self.right(),
            Egocentric::Backward => self.reverse(),
            Egocentric::Left => self.left(),
        }
    }

    pub const fn rotate_vec<U>(&self, v: Vector2D<isize, U>) -> Vector2D<isize, U> {
        match self {
            Egocentric::Forward => v,
            Egocentric::Right => vec2(-v.y, v.x),
            Egocentric::Backward => vec2(-v.x, -v.y),
            Egocentric::Left => vec2(v.y, -v.x),
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

impl IdLike for Egocentric {
    fn id_min_value() -> Self {
        Egocentric::Forward
    }

    fn id_max_value() -> Self {
        Egocentric::Left
    }
}