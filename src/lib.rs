mod cardinal;
mod egocentric;
mod egosphere;
mod geom;
mod portals;
// mod vision;

pub use geom::*;

pub(crate) use cardinal::Cardinal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
