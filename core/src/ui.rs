use std::collections::HashMap;

pub type WidgetId = u32;

#[derive(Debug, thiserror::Error)]
pub enum UiError {}

#[derive(Debug, Default)]
pub struct Ui {
    widgets: HashMap<WidgetId, WidgetNode>,
}

impl Ui {}

#[derive(Debug, Clone)]
pub struct WidgetNode {}
