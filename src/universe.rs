use std::collections::HashMap;

use moogle::*;

use crate::portals::Portals;

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

pub struct RTRoom {

}