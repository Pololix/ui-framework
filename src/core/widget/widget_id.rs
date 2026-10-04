pub type WidgetId = u32;

#[derive(Debug, Default)]
pub struct WidgetIdAllocator(WidgetId);

impl WidgetIdAllocator {
    pub fn next(&mut self) -> WidgetId {
        let id = self.0;
        self.0 += 1;

        id
    }
}
