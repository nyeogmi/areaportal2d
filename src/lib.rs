mod cardinal;
mod egocentric;
mod egosphere;
mod geom;
mod portals;
mod universe;
// mod vision;

pub use geom::*;

pub(crate) use cardinal::Cardinal;
pub(crate) use egocentric::Egocentric;
pub(crate) use portals::Portals;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
