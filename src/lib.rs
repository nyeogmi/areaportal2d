mod cardinal;
mod egocentric;
mod egosphere;
mod geom;
mod portals;
// mod vision;

pub use geom::{
    GlobalView, GlobalPoint,
    EgoPoint, EgoRect, EgoSize, EgoSpace, EgoVec
};

pub use cardinal::Cardinal;
pub use egocentric::Egocentric;
pub use egosphere::{EgoWindow, Egosphere};
pub use portals::Portals;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
