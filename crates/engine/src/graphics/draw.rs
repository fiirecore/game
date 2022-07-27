pub struct Draw<'a>(pub wgpu::RenderPass<'a>);

impl<'a> From<wgpu::RenderPass<'a>> for Draw<'a> {
    fn from(pass: wgpu::RenderPass<'a>) -> Self {
        Self(pass)
    }
}