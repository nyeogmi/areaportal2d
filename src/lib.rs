mod cardinal;
mod egocentric;
mod egosphere;
mod geom;
mod portals;
mod viewport;

pub use geom::{
    GlobalView, GlobalPoint,
    EgoPoint, EgoRect, EgoSize, EgoSpace, EgoVec
};

pub use cardinal::Cardinal;
pub use egocentric::Egocentric;
pub use egosphere::Egosphere;
pub use portals::Portals;
pub use viewport::Viewport;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
