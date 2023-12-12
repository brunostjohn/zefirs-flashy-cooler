use ultralight_sys::ULGPUState;

use crate::error::ULError;

use super::{from_ul_arr, rect::Rect, shader_type::ShaderType};

#[derive(Debug)]
pub struct GPUState {
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub transform: [f32; 16],
    pub enable_texturing: bool,
    pub enable_blend: bool,
    pub shader_type: ShaderType,
    pub render_buffer_id: u32,
    pub texture_1_id: Option<u32>,
    pub texture_2_id: Option<u32>,
    pub texture_3_id: Option<u32>,
    pub uniform_scalar: [f32; 8],
    pub uniform_vector: [[f32; 4]; 8],
    pub clip_size: u8,
    pub clip: [[[f32; 4]; 4]; 8],
    pub enable_scissor: bool,
    pub scissor_rect: Rect<i32>,
}

impl TryFrom<ULGPUState> for GPUState {
    type Error = ULError;

    #[inline]
    fn try_from(gs: ULGPUState) -> Result<Self, Self::Error> {
        Ok(GPUState {
            viewport_width: gs.viewport_width,
            viewport_height: gs.viewport_height,
            transform: gs.transform.data,
            enable_texturing: gs.enable_texturing,
            enable_blend: gs.enable_blend,
            shader_type: ShaderType::try_from(gs.shader_type as i32)?,
            render_buffer_id: gs.render_buffer_id,
            texture_1_id: if gs.texture_1_id == 0 {
                None
            } else {
                Some(gs.texture_1_id)
            },
            texture_2_id: if gs.texture_2_id == 0 {
                None
            } else {
                Some(gs.texture_2_id)
            },
            texture_3_id: if gs.texture_3_id == 0 {
                None
            } else {
                Some(gs.texture_3_id)
            },
            uniform_scalar: gs.uniform_scalar,
            uniform_vector: from_ul_arr!(gs.uniform_vector, value),
            clip_size: gs.clip_size,
            clip: from_ul_arr!(mat gs.clip, data),
            enable_scissor: gs.enable_scissor,
            scissor_rect: Rect::from(gs.scissor_rect),
        })
    }
}
