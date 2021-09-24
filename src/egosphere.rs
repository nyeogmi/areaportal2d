use std::collections::VecDeque;

use euclid::{Vector2D, rect, vec2};
use gridd_euclid::Grid;

use crate::{EgoPoint, EgoRect, EgoSpace, EgoVec, GlobalPoint, GlobalView, portals::Portals, universe::RTUniverse};

// TODO: Refuse to construct this unless the observer is, in fact, in the rectangle
pub struct EgoWindow {
    rect: EgoRect,
    observer_in_rect: EgoPoint,
    observer: GlobalView,
}

pub struct Egosphere {
    // TODO: Don't use Options in the future
    global: Grid<Option<GlobalView>, EgoSpace>,
    basis: Grid<Option<EgoPoint>, EgoSpace>,
    dirty_global: Grid<u64, EgoSpace>,
    dirty_basis: Grid<u64, EgoSpace>,
    dirty_token: u64,
    explore: VecDeque<EgoPoint>,
}

impl Egosphere {
    pub fn new() -> Egosphere {
        let zero = rect(0, 0, 0, 0);
        Egosphere {
            global: Grid::new(zero, || unreachable!()),
            basis: Grid::new(zero, ||unreachable!()),
            dirty_global: Grid::new(zero, ||unreachable!()),
            dirty_basis: Grid::new(zero, ||unreachable!()),
            dirty_token: 0,
            explore: VecDeque::new(),
        }
    }

    fn resize(&mut self, rect: EgoRect) {
        if self.global.rect() == rect { return; }
        
        self.global.resize(rect, || None);
        self.basis.resize(rect, || None);
        let tok = self.dirty_token;
        self.dirty_global.resize(rect, || tok);
        self.dirty_basis.resize(rect, || tok);

        self.dirty_token += 1;
    }

    pub fn calculate(&mut self, window: EgoWindow, universe: RTUniverse, blocked: impl Fn(GlobalView) -> bool) {
        // TODO: Take a function to check if this is blocked, as an arg
        self.resize(window.rect);

        self.explore.clear();
        self.dirty_token += 1;

        let view = window.observer_in_rect;
        self.global.set(view, Some(window.observer));

        // == starting point ==
        self.enqueue_prio(view, vec2(0, 0));

        // == dijkstra away from the starting point ==
        while let Some(v) = self.explore.pop_front() {
            if self.dirty_global.get(v).unwrap() == &self.dirty_token { continue; }

            let basis = self.basis.get(v).unwrap().unwrap();
            let previous = self.global.get(basis).unwrap().unwrap();
            let real: GlobalView = universe.step_offset(previous, v - basis);

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

    pub fn at(&self, v: EgoPoint) -> Option<GlobalView> {
        if let Some(Some(p)) = self.global.get(v) {
            if let Some(&tok) = self.dirty_basis.get(v) {
                if tok == self.dirty_token {
                    return Some(*p)
                }
            }
        }
        return None
    }
}