use crate::types::Color;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    width: Dimension,
    height: Dimension,

    margin: Edges,
    padding: Edges,
    vertical_align: VerticalAlign,
    horizontal_align: HorizontalAlign,

    fg_color: Option<Color>,
    bg_color: Option<Color>,
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Px(f32),
    Percent(f32),
    Fill,
}

impl Default for Dimension {
    fn default() -> Self {
        Self::Px(0.0)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Edges {
    top: Dimension,
    bottom: Dimension,
    left: Dimension,
    right: Dimension,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum ChildAlign {
    Row,
    Column,
}
