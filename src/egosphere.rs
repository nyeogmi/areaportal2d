use std::collections::VecDeque;

use euclid::{point2, rect, vec2};
use gridd_euclid::Grid;
use moogle::IdLike;

use crate::{EgoPoint, EgoRect, EgoSpace, EgoVec, GlobalView, Viewport, portals::Portals};

pub struct Egosphere<R: IdLike> {
    // TODO: Don't use Options in the future?
    global: Grid<Option<GlobalView<R>>, EgoSpace>,
    basis: Grid<Option<EgoPoint>, EgoSpace>,
    dirty_global: Grid<u64, EgoSpace>,
    dirty_basis: Grid<u64, EgoSpace>,
    dirty_token: u64,
    fov: Grid<u64, EgoSpace>,  // if less than dirty token, then not visible this round
    fullbright: bool,

    explore: VecDeque<EgoPoint>,
}

impl<R: IdLike> Egosphere<R> {
    pub fn new(fullbright: bool) -> Egosphere<R> {
        let zero = rect(0, 0, 0, 0);
        Egosphere {
            global: Grid::new(zero, || unreachable!()),
            basis: Grid::new(zero, ||unreachable!()),
            dirty_global: Grid::new(zero, ||unreachable!()),
            dirty_basis: Grid::new(zero, ||unreachable!()),
            dirty_token: 0,
            fov: Grid::new(zero, ||unreachable!()),
            fullbright,
            explore: VecDeque::new(),
        }
    }

    pub fn calculate(&mut self, viewport: Viewport<R>, portals: Portals<R>, blocked: impl Fn(GlobalView<R>) -> bool) {
        self.resize(viewport.rect);
        self.dirty_token += 1;

        self.calculate_globalmap(viewport, portals);
        self.calculate_fov(viewport, blocked);
    }

    fn resize(&mut self, rect: EgoRect) {
        if self.global.rect() == rect { return; }
        
        self.global.resize(rect, || None);
        self.basis.resize(rect, || None);
        let tok = self.dirty_token;
        self.dirty_global.resize(rect, || tok);
        self.dirty_basis.resize(rect, || tok);

        self.fov.resize(rect, || tok);
    }

    fn calculate_globalmap(&mut self, viewport: Viewport<R>, portals: Portals<R>) {
        self.explore.clear();

        let view = viewport.observer_in_rect;
        self.global.set(view, Some(viewport.observer));

        // == starting point ==
        self.enqueue_prio(view, vec2(0, 0));

        // == dijkstra away from the starting point ==
        while let Some(v) = self.explore.pop_front() {
            if self.dirty_global.get(v).unwrap() == &self.dirty_token { continue; }

            let basis = self.basis.get(v).unwrap().unwrap();
            let previous = self.global.get(basis).unwrap().unwrap();
            let real: GlobalView<R> = portals.step_offset(previous, v - basis);

            self.global.set(v, Some(real));
            self.dirty_global.set(v, self.dirty_token);

            self.enqueue_prio(v, vec2(0, -1));
            self.enqueue_prio(v, vec2(0, 1));
            self.enqueue_prio(v, vec2(-1, 0));
            self.enqueue_prio(v, vec2(1, 0));
            self.enqueue(v, vec2(-1, -1));
            self.enqueue(v, vec2(-1, 1));
            self.enqueue(v, vec2(1, -1));
            self.enqueue(v, vec2(1, 1));
        }
    }

    fn enqueue_prio(&mut self, v: EgoPoint, offset: EgoVec) {
        let v2 = v + offset;
        if !self.global.contains(v2) { return; }

        self.basis.set(v2, Some(v));
        self.dirty_basis.set(v2, self.dirty_token);
        self.explore.push_back(v2)
    }

    fn enqueue(&mut self, v: EgoPoint, offset: EgoVec) {
        let v2 = v + offset;
        if !self.global.contains(v2) { return; }

        if self.dirty_basis.get(v2) != Some(&self.dirty_token) {
            self.basis.set(v2, Some(v));
            self.dirty_basis.set(v2, self.dirty_token);
            self.explore.push_back(v2)
        }
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
            if let Some(&tok) = self.dirty_basis.get(v) {
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