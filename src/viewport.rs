use moogle::IdLike;

use crate::{EgoPoint, EgoRect, GlobalView};

#[derive(Clone, Copy)]
pub struct Viewport<R: IdLike> {
    pub rect: EgoRect,
    pub observer_in_rect: EgoPoint,
    pub observer: GlobalView<R>,
}
