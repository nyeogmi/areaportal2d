use moogle::IdLike;

use crate::{EgoPoint, EgoRect, GlobalView};

#[derive(Clone, Copy)]
pub struct Viewport<R: IdLike> {
    pub(crate) rect: EgoRect,
    pub(crate) observer_in_rect: EgoPoint,
    pub(crate) observer: GlobalView<R>,
}
