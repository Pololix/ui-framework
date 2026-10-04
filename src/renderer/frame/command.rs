use crate::{
    renderer::frame::RenderId,
    types::{Color, Rect, Viewport},
};

#[derive(Debug, Clone)]
pub(crate) enum RenderCommand {
    Resize(Viewport),
    ChangeScaleFactor(f32),
    RedrawFrame,
    ClearFrame,

    Quad {
        id: RenderId,
        rect: Rect,
        color: Color,
    },

    Remove(RenderId),
}
