use crate::types::Rect;

#[derive(Debug)]
pub enum FrameInvalidation {
    Empty,
    Partial(Rect),
    Full,
}

impl Default for FrameInvalidation {
    fn default() -> Self {
        Self::Empty
    }
}

impl FrameInvalidation {
    pub fn empty(&mut self) {
        *self = Self::Empty;
    }

    pub fn partial(&mut self, new: Rect) {
        match self {
            Self::Empty => *self = Self::Partial(new),
            Self::Partial(rect) => {
                rect.union(new);
            }
            Self::Full => {}
        }
    }

    pub fn full(&mut self) {
        *self = Self::Full;
    }
}
