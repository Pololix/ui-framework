use crate::{
    frame::{frame_invalidation::FrameInvalidation, render_id::RenderId},
    quad::Quad,
    types::{Rect, Viewport},
};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Frame {
    invalidation: FrameInvalidation,
    quads: HashMap<RenderId, Vec<Quad>>,
}

impl Frame {
    pub fn clear(&mut self) {
        self.quads.clear();
    }

    pub fn empty_invalidation(&mut self) {
        self.invalidation.empty();
    }

    pub fn full_invalidation(&mut self) {
        self.invalidation.full();
    }

    pub fn upload(&mut self, id: RenderId, quads: &[Quad]) {
        // remove previous render items and insert new ones
        self.remove(id);

        // invalidate newly occupied space
        for quad in quads {
            self.invalidation.partial(quad.as_rect());
        }
        self.quads.insert(id, quads.to_vec());
    }

    pub fn remove(&mut self, id: RenderId) {
        // invalidate previously occupied space
        if let Some(quads) = self.quads.remove(&id) {
            for quad in quads {
                self.invalidation.partial(quad.as_rect());
            }
        };
    }

    pub fn get_quads(&mut self, viewport: Viewport) -> Option<(Rect, Vec<Quad>)> {
        let (rect, quads) = match self.invalidation {
            FrameInvalidation::Empty => {
                return None;
            }
            FrameInvalidation::Partial(rect) => (
                rect,
                self.quads
                    .values()
                    .flatten()
                    .filter(|quad| quad.as_rect().intersects(rect))
                    .copied()
                    .collect(),
            ),
            FrameInvalidation::Full => (
                viewport.as_rect(),
                self.quads.values().flatten().copied().collect(),
            ),
        };

        self.invalidation.empty();

        Some((rect, quads))
    }
}
