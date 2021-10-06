use euclid::{point2, rect, vec2};
use gridd_euclid::Grid;
use moogle::IdLike;

use crate::{EgoPoint, EgoRect, EgoSpace, EgoVec, GlobalView, Viewport, portals::Portals};

use std::collections::binary_heap::BinaryHeap;

pub struct Egosphere<R: IdLike> {
    // TODO: Don't use Options in the future?
    global: Grid<Option<GlobalView<R>>, EgoSpace>,
    dirty_global: Grid<u64, EgoSpace>,
    dirty_token: u64,
    fov: Grid<u64, EgoSpace>,  // if less than dirty token, then not visible this round
    fullbright: bool,  // if set, don't check fov at all

    explore: BinaryHeap<(isize, (isize, isize), (isize, isize))>,  
}

impl<R: IdLike> Egosphere<R> {
    pub fn new(fullbright: bool) -> Egosphere<R> {
        let zero = rect(0, 0, 0, 0);
        Egosphere {
            global: Grid::new(zero, || unreachable!()),
            dirty_global: Grid::new(zero, ||unreachable!()),
            dirty_token: 0,
            fov: Grid::new(zero, ||unreachable!()),
            fullbright,
            explore: BinaryHeap::new(),
        }
    }

    pub fn calculate(&mut self, viewport: Viewport<R>, portals: &Portals<R>, blocked: impl Fn(GlobalView<R>) -> bool) {
        self.resize(viewport.rect);
        self.dirty_token += 1;

        self.calculate_globalmap(viewport, portals, |x| blocked(x));
        self.calculate_fov(viewport, blocked);
    }

    fn resize(&mut self, rect: EgoRect) {
        if self.global.rect() == rect { return; }
        
        self.global.resize(rect, || None);
        let tok = self.dirty_token;
        self.dirty_global.resize(rect, || tok);

        self.fov.resize(rect, || tok);
    }

    fn calculate_globalmap(&mut self, viewport: Viewport<R>, portals: &Portals<R>, blocked: impl Fn(GlobalView<R>) -> bool) {
        self.explore.clear();

        let view = viewport.observer_in_rect;
        self.global.set(view, Some(viewport.observer));

        // == starting point ==
        self.enqueue(0, view, vec2(0, 0));

        // == dijkstra away from the starting point ==
        while let Some((cost, (dst_x, dst_y), (src_x, src_y))) = self.explore.pop() {
            let src: EgoPoint = point2(src_x, src_y);
            let dst: EgoPoint = point2(dst_x, dst_y);

            if self.dirty_global.get(dst).unwrap() == &self.dirty_token { continue; }

            let global_src = self.global.get(src).unwrap().unwrap();
            let global_dst: GlobalView<R> = portals.step_offset(global_src, dst - src);

            self.global.set(dst, Some(global_dst));
            self.dirty_global.set(dst, self.dirty_token);

            if blocked(global_dst) { continue; }

            self.enqueue(cost - 1, dst, vec2(0, -1));
            self.enqueue(cost - 1, dst, vec2(0, 1));
            self.enqueue(cost - 1, dst, vec2(-1, 0));
            self.enqueue(cost - 1, dst, vec2(1, 0));
            self.enqueue(cost - 4, dst, vec2(-1, -1));
            self.enqueue(cost - 4, dst, vec2(-1, 1));
            self.enqueue(cost - 4, dst, vec2(1, -1));
            self.enqueue(cost - 4, dst, vec2(1, 1));
        }
    }

    fn enqueue(&mut self, cost: isize, src: EgoPoint, offset: EgoVec) {
        let dst = src + offset;
        if !self.global.contains(dst) { return; }
        if self.dirty_global.get(dst) == Some(&self.dirty_token) { return }

        self.explore.push((cost, (dst.x, dst.y), (src.x, src.y)))
    }

    fn calculate_fov(&mut self, viewport: Viewport<R>, blocked: impl Fn(GlobalView<R>) -> bool) {  
        if self.fullbright { return; }

        let mut fov_tmp = Grid::new(rect(0, 0, 0, 0), || unreachable!()); 
        std::mem::swap(&mut self.fov, &mut fov_tmp);
        symmetric_shadowcasting::compute_fov(
            (viewport.observer_in_rect.x, viewport.observer_in_rect.y), 
            &mut |(x, y)| { 
                let v = point2(x, y);
                if let Some(p) = self.at_fullbright(v) {
                    return blocked(p)
                }
                true
            }, 
            &mut |(x, y)| {
                let p = point2(x, y);
                if viewport.rect.contains(p) {
                    fov_tmp.set(p, self.dirty_token)
                }
            }
        );
        std::mem::swap(&mut self.fov, &mut fov_tmp);
    }

    pub fn at(&self, v: EgoPoint) -> Option<GlobalView<R>> {
        if self.fullbright {
            return self.at_fullbright(v)
        }
        return self.at_fov(v)
    }

    fn at_fullbright(&self, v: EgoPoint) -> Option<GlobalView<R>> {
        // ignore FOV
        if let Some(Some(p)) = self.global.get(v) {
            if let Some(&tok) = self.dirty_global.get(v) {
                if tok == self.dirty_token {
                    return Some(*p);
                }
            }
        }
        None
    }

    fn at_fov(&self, v: EgoPoint) -> Option<GlobalView<R>> {
        // ignore FOV
        if let Some(&fov_tok) = self.fov.get(v) {
            if fov_tok == self.dirty_token {
                if let Some(v) = self.at_fullbright(v) {
                    return Some(v);
                }
            }
        }
        None
    }
}