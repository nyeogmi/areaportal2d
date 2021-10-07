use moogle::*;

use crate::{EgoVec, GlobalView, egocentric::{Egocentric}};

pub struct Portals<R: IdLike> {
    owner: RawManyToOne<GlobalView<R>, AreaPortal<R>>,
    traps: RawOneToOne<GlobalView<R>, GlobalView<R>>,
}

impl<R: IdLike> Portals<R> {
    pub fn new() -> Self {
        Portals {
            owner: RawManyToOne::new(),
            traps: RawOneToOne::new(),
        }
    }
}


#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AreaPortal<R: IdLike> {
    pub src: GlobalView<R>,
    pub dst: GlobalView<R>,
    pub size: isize,  // C# comment said: "NOTE: Ignored for any areaportal that is already reified"
}

impl<R: IdLike> IdLike for AreaPortal<R> {
    fn id_min_value() -> Self {
        AreaPortal { src: GlobalView::id_min_value(), dst: GlobalView::id_min_value(), size: isize::id_min_value() }
    }

    fn id_max_value() -> Self {
        AreaPortal { src: GlobalView::id_max_value(), dst: GlobalView::id_max_value(), size: isize::id_max_value() }
    }
}

impl<R: IdLike> AreaPortal<R> {
    fn reverse(&self) -> AreaPortal<R> {
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

impl<R: IdLike> Portals<R> {
    pub fn add_area_portal(&mut self, ap: AreaPortal<R>) {
        if self.owner.bwd().contains_key(ap) {
            return; // already exists
        }

        self.remove_overlapping_portals(ap);
        self.remove_overlapping_portals(ap.reverse());

        self.actually_add_one_way_area_portal(ap);
        self.actually_add_one_way_area_portal(ap.reverse());
    }

    pub fn remove_area_portal(&mut self, ap: AreaPortal<R>) {
        for cell in self.owner.bwd().get(ap).iter() {
            self.traps.mut_fwd().expunge(cell);
            self.traps.mut_bwd().expunge(cell);
        }
        for cell in self.owner.bwd().get(ap.reverse()).iter() {
            self.traps.mut_bwd().expunge(cell);
        }
        self.owner.mut_bwd().expunge(ap);
    }

    fn remove_overlapping_portals(&mut self, ap: AreaPortal<R>) {
        let src_fwd = ap.src.c;
        let dst_fwd = ap.dst.c;

        for i in 0..ap.size {
            let src_xy = ap.src.x + src_fwd.right().offset_by(i) + src_fwd.offset();
            let dst_xy = ap.dst.x + dst_fwd.right().offset_by(i);

            let src = GlobalView { r: ap.src.r, x: src_xy, c: src_fwd };
            let dst = GlobalView { r: ap.dst.r, x: dst_xy, c: dst_fwd };

            if let Some(owner) = self.owner.fwd().get(src) {
                self.remove_area_portal(owner)
            }
            if let Some(owner) = self.owner.fwd().get(dst) {
                self.remove_area_portal(owner)
            }
        }
    }

    fn actually_add_one_way_area_portal(&mut self, ap: AreaPortal<R>) {
        let src_fwd = ap.src.c;
        let dst_fwd = ap.dst.c;

        for i in 0..ap.size {
            let src_xy = ap.src.x + src_fwd.right().offset_by(i) + src_fwd.offset();
            let dst_xy = ap.dst.x + dst_fwd.right().offset_by(i);

            let src = GlobalView { r: ap.src.r, x: src_xy, c: src_fwd };
            let dst = GlobalView { r: ap.dst.r, x: dst_xy, c: dst_fwd };

            // println!("trapping: {:?}x{:?} => {:?}x{:?}", src_xy, src_fwd, dst_xy, dst_fwd);

            self.traps.mut_fwd().insert(src, dst);
            self.owner.mut_fwd().insert(src, ap);
            self.owner.mut_fwd().insert(dst, ap);
        }
    }

    pub fn step_offset(&self, src: GlobalView<R>, ego: EgoVec) -> (bool, GlobalView<R>) {
        assert!((-1..=1).contains(&ego.x));
        assert!((-1..=1).contains(&ego.y));

        if ego.x == 0 && ego.y == 0 {
            return (false, src);
        }

        match (ego.x, ego.y) {
            (0, -1) => self.step_directional(src, Egocentric::Forward),
            (1, 0) => self.step_directional(src, Egocentric::Right),
            (0, 1) => self.step_directional(src, Egocentric::Backward),
            (-1, 0) => self.step_directional(src, Egocentric::Left),

            (-1, -1) => self.step_directional_2(src, Egocentric::Forward, Egocentric::Left),
            (1, -1) => self.step_directional_2(src, Egocentric::Forward, Egocentric::Right),
            (-1, 1) => self.step_directional_2(src, Egocentric::Backward, Egocentric::Left),
            (1, 1) => self.step_directional_2(src, Egocentric::Backward, Egocentric::Right),
            _ => unreachable!(),
        }
    }

    pub fn step_directional(&self, src: GlobalView<R>, ego: Egocentric) -> (bool, GlobalView<R>) {
        let (portal, v2) = self.step_forward(src.rotated(ego));
        (portal, v2.rotated(ego.undo()))
    }

    pub fn step_directional_2(&self, src: GlobalView<R>, step1: Egocentric, step2: Egocentric) -> (bool, GlobalView<R>) {
        // maximize number of portals stepped through
        let (portal1_1, mid1) = self.step_directional(src, step1);
        let (portal2_1, out1) = self.step_directional(mid1, step2);

        let (portal1_2, mid2) = self.step_directional(src, step2);
        let (portal2_2, out2) = self.step_directional(mid2, step1);

        let psum_1 = (if portal1_1 { 1 } else { 0 }) + (if portal2_1 { 1 } else { 0 });
        let psum_2 = (if portal1_2 { 1 } else { 0 }) + (if portal2_2 { 1 } else { 0 });

        if psum_1 >= psum_2 {
            return (psum_1 > 0, out1)
        } else {
            return (psum_2 > 0, out2)
        }
    }

    pub fn step_forward(&self, src: GlobalView<R>) -> (bool, GlobalView<R>) {
        let dst_normal = GlobalView { 
            r: src.r,
            x: src.x + src.c.offset(),
            c: src.c,
        };

        if let Some(trap) = self.traps.fwd().get(dst_normal) {
            (true, trap)
        } else {
            (false, dst_normal)
        }
    }
}