use crate::types::Rect;

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    rect: Rect,
    content_rect: Rect,
}
