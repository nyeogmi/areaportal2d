use std::collections::HashMap;

use moogle::*;

use crate::{EgoVec, GlobalView, egocentric::Egocentric, portals::Portals};

pub struct ULUniverse {
    // TODO: Only Moogle these when loaded at runtime
    /*
    zones: RawPom<ULZone>,
    rooms: RawPom<ULRoom>,

    zone_rooms: RawOneToMany<Id<ULZone>, Id<ULRoom>>,
    */
}

pub struct ULZone {

}

pub struct ULRoom {

}

pub struct RTUniverse {
    unloaded: ULUniverse,

    loaded_rooms: HashMap<Id<ULRoom>, RTRoom>,
    portals: Portals,
}

impl RTUniverse {
    pub fn step_offset(&self, src: GlobalView, ego: EgoVec) -> GlobalView {
        self.portals.step_offset(src, ego)
    }

    pub fn step_directional(&self, src: GlobalView, ego: Egocentric) -> GlobalView {
        self.portals.step_directional(src, ego)
    }

    pub fn step_forward(&self, src: GlobalView) -> GlobalView {
        self.portals.step_forward(src)
    }
}

pub struct RTRoom {

}