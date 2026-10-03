pub type RenderId = u32;

#[derive(Debug, Default)]
pub struct RenderIdAlloc(RenderId);

impl RenderIdAlloc {
    pub fn next(&mut self) -> RenderId {
        let id = self.0;
        self.0 += 1;

        id
    }
}
