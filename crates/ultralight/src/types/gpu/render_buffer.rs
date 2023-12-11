use ultralight_sys::ULRenderBuffer;

#[derive(Debug)]
pub struct RenderBuffer {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub has_stencil_buffer: bool,
    pub has_depth_buffer: bool,
}

impl From<ULRenderBuffer> for RenderBuffer {
    #[inline(always)]
    fn from(rb: ULRenderBuffer) -> Self {
        RenderBuffer {
            texture_id: rb.texture_id,
            width: rb.width,
            height: rb.height,
            has_stencil_buffer: rb.has_stencil_buffer,
            has_depth_buffer: rb.has_depth_buffer,
        }
    }
}
