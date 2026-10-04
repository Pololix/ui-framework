use crate::{
    core::{Style, layout::Layout},
    types::Viewport,
    widget::{Widget, WidgetId, WidgetIdAllocator},
};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum UiError {}

pub struct Ui {
    viewport: Viewport,
    scale_factor: f32,
    ui_scale: f32,

    widget_ids: WidgetIdAllocator,
    widgets: HashMap<WidgetId, WidgetNode>,
}

impl Ui {
    pub fn new(viewport: Viewport, scale_factor: f32, ui_scale: f32) -> Self {
        Self {
            viewport,
            scale_factor,
            ui_scale,

            widget_ids: WidgetIdAllocator::default(),
            widgets: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct WidgetNode {
    parent: Option<WidgetId>,
    children: Vec<WidgetId>,

    widget: Box<dyn Widget>,
    style: Style,
    layout: Layout,
}
