#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    Px(f32),
    Percent(f32),
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

impl Edges {
    pub fn all(mut self, spacing: Dimension) -> Self {
        self.top = spacing;
        self.bottom = spacing;
        self.left = spacing;
        self.right = spacing;

        self
    }
}
