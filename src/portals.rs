use moogle::*;

use crate::{GlobalPoint, GlobalView, cardinal::Cardinal, egocentric::{self, Egocentric}};

pub(crate) struct Portals {
    traps: RawToOne<GlobalView, GlobalView>,
}


#[derive(Clone, Copy)]
pub struct AreaPortal {
    src: GlobalView,
    dst: GlobalView,
    size: isize,  // C# comment said: "NOTE: Ignored for any areaportal that is already reified"
}

impl AreaPortal {
    fn reverse(&self) -> AreaPortal {
        AreaPortal {
            src: GlobalView {
                r: self.dst.r,
                x: self.dst.x + self.dst.c.right().offset_by(self.size - 1),
                c: self.dst.c.reverse(),
            },
            dst: GlobalView {
                r: self.src.r,
                x: self.src.x + self.src.c.right().offset_by(self.size - 1),
                c: self.src.c.reverse(),
            },
            size: self.size
        }
    }
}

impl Portals {
    fn add_area_portal(&mut self, ap: AreaPortal) {
        // NYEO NOTE: This function was unused in C#
        self.add_one_way_area_portal(ap);
        self.add_one_way_area_portal(ap.reverse());
    }

    fn add_one_way_area_portal(&mut self, ap: AreaPortal) {
        let src_fwd = ap.src.c;
        let dst_fwd = ap.dst.c;

        for i in 0..ap.size {
            let src_xy = ap.src.x + src_fwd.right().offset_by(i) + src_fwd.offset();
            let dst_xy = ap.dst.x + dst_fwd.right().offset_by(i);

            self.traps.mut_fwd().insert(
                GlobalView { r: ap.src.r, x: src_xy, c: src_fwd },
                GlobalView { r: ap.dst.r, x: dst_xy, c: dst_fwd },
            );
        }
    }

    fn step(&self, src: GlobalView, ego: Egocentric) -> GlobalView {
        self.step_forward(src.rotated(ego)).rotated(ego.reverse())
    }

    fn step_forward(&self, src: GlobalView) -> GlobalView {
        let dst_normal = GlobalView { 
            r: src.r,
            x: src.x + src.c.offset(),
            c: src.c,
        };

        if let Some(trap) = self.traps.fwd().get(dst_normal) {
            trap
        } else {
            dst_normal
        }
    }
}