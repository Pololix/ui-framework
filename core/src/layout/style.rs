use crate::layout::{Dimension, Edges};

#[derive(Debug, Clone, Copy)]
pub struct WidgetStyle {
    pub width: Dimension,
    pub height: Dimension,
    pub margin: Edges,
    pub padding: Edges,
}
