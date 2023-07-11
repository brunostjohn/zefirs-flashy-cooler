use std::{convert::TryFrom, slice};

use ul_sys::*;

#[path = "./bitmap.rs"]
pub mod bitmap;
pub use bitmap::*;

pub trait GPUDriver {
    fn begin_synchronize(&mut self);
    fn end_synchronize(&mut self);
    fn next_texture_id(&mut self) -> u32;
    fn create_texture(&mut self, texture_id: u32, bitmap: OwnedBitmap);
    fn update_texture(&mut self, texture_id: u32, bitmap: OwnedBitmap);
    fn destroy_texture(&mut self, texture_id: u32);
    fn next_render_buffer_id(&mut self) -> u32;
    fn create_render_buffer(&mut self, render_buffer_id: u32, render_buffer: RenderBuffer);
    fn destroy_render_buffer(&mut self, render_buffer_id: u32);
    fn next_geometry_id(&mut self) -> u32;
    fn create_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: VertexBuffer,
        index_buffer: IndexBuffer,
    );
    fn update_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: VertexBuffer,
        index_buffer: IndexBuffer,
    );
    fn destroy_geometry(&mut self, geometry_id: u32);
    fn update_command_list(&mut self, command_list: Vec<GPUCommand>);
}

macro_rules! from_ul_arr {
    ($arr:expr, $from:ident) => {
        [
            $arr[0].$from,
            $arr[1].$from,
            $arr[2].$from,
            $arr[3].$from,
            $arr[4].$from,
            $arr[5].$from,
            $arr[6].$from,
            $arr[7].$from,
        ]
    };
    (mat $arr:expr, $from:ident) => {
        [
            from_ul_arr!(mat $arr[0].$from),
            from_ul_arr!(mat $arr[1].$from),
            from_ul_arr!(mat $arr[2].$from),
            from_ul_arr!(mat $arr[3].$from),
            from_ul_arr!(mat $arr[4].$from),
            from_ul_arr!(mat $arr[5].$from),
            from_ul_arr!(mat $arr[6].$from),
            from_ul_arr!(mat $arr[7].$from),
        ]
    };
    (mat $arr: expr) => {
        [
            [$arr[0], $arr[1], $arr[2], $arr[3]],
            [$arr[4], $arr[5], $arr[6], $arr[7]],
            [$arr[8], $arr[9], $arr[10], $arr[11]],
            [$arr[12], $arr[13], $arr[14], $arr[15]],
        ]
    };
}

pub struct RenderBuffer {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub has_stencil_buffer: bool,
    pub has_depth_buffer: bool,
}

impl From<ULRenderBuffer> for RenderBuffer {
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

#[allow(non_camel_case_types)]
pub enum VertexBufferFormat {
    Format_2f_4ub_2f = ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f as isize,
    Format_2f_4ub_2f_2f_28f = ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f_2f_28f as isize,
}

impl TryFrom<ULVertexBufferFormat> for VertexBufferFormat {
    type Error = ();

    #[allow(non_upper_case_globals)]
    fn try_from(vbf: ULVertexBufferFormat) -> Result<Self, Self::Error> {
        match vbf {
            ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f => {
                Ok(VertexBufferFormat::Format_2f_4ub_2f)
            }
            ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f_2f_28f => {
                Ok(VertexBufferFormat::Format_2f_4ub_2f_2f_28f)
            }
            _ => Err(()),
        }
    }
}

pub struct VertexBuffer {
    pub format: VertexBufferFormat,
    pub buffer: Vec<u8>,
}

impl TryFrom<ULVertexBuffer> for VertexBuffer {
    type Error = ();

    fn try_from(vb: ULVertexBuffer) -> Result<Self, Self::Error> {
        if vb.data.is_null() {
            return Err(());
        }
        let format = VertexBufferFormat::try_from(vb.format)?;
        let buffer = unsafe { slice::from_raw_parts(vb.data, vb.size as usize) };
        Ok(VertexBuffer {
            format,
            buffer: buffer.to_vec(),
        })
    }
}

pub struct IndexBuffer {
    pub buffer: Vec<u32>,
}

impl From<ULIndexBuffer> for IndexBuffer {
    fn from(vb: ULIndexBuffer) -> Self {
        assert!(vb.size % 4 == 0);
        assert!(!vb.data.is_null());
        let index_slice = unsafe { slice::from_raw_parts(vb.data as _, vb.size as usize / 4) };
        IndexBuffer {
            buffer: index_slice.to_vec(),
        }
    }
}

pub enum ShaderType {
    Fill = ULShaderType_kShaderType_Fill as isize,
    FillPath = ULShaderType_kShaderType_FillPath as isize,
}

impl TryFrom<ULShaderType> for ShaderType {
    type Error = ();

    #[allow(non_upper_case_globals)]
    fn try_from(st: ULShaderType) -> Result<Self, Self::Error> {
        match st {
            ULShaderType_kShaderType_Fill => Ok(ShaderType::Fill),
            ULShaderType_kShaderType_FillPath => Ok(ShaderType::FillPath),
            _ => Err(()),
        }
    }
}

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
    type Error = ();

    fn try_from(gs: ULGPUState) -> Result<Self, Self::Error> {
        Ok(GPUState {
            viewport_width: gs.viewport_width,
            viewport_height: gs.viewport_height,
            transform: gs.transform.data,
            enable_texturing: gs.enable_texturing,
            enable_blend: gs.enable_blend,
            shader_type: ShaderType::try_from(gs.shader_type as u32)?,
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

pub enum GPUCommand {
    ClearRenderBuffer {
        render_buffer_id: u32,
    },
    DrawGeometry {
        gpu_state: Box<GPUState>,
        geometry_id: u32,
        indices_offset: u32,
        indices_count: u32,
    },
}

impl TryFrom<ULCommand> for GPUCommand {
    type Error = ();

    #[allow(non_upper_case_globals)]
    fn try_from(gc: ULCommand) -> Result<Self, Self::Error> {
        match gc.command_type as u32 {
            ULCommandType_kCommandType_DrawGeometry => Ok(GPUCommand::DrawGeometry {
                gpu_state: Box::new(GPUState::try_from(gc.gpu_state)?),
                geometry_id: gc.geometry_id,
                indices_count: gc.indices_count,
                indices_offset: gc.indices_offset,
            }),
            ULCommandType_kCommandType_ClearRenderBuffer => Ok(GPUCommand::ClearRenderBuffer {
                render_buffer_id: gc.gpu_state.render_buffer_id,
            }),
            _ => Err(()),
        }
    }
}

pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

impl Rect<i32> {
    pub fn is_empty(&self) -> bool {
        self.left == 0 && self.top == 0 && self.right == 0 && self.bottom == 0
    }
}

impl From<ULRect> for Rect<f32> {
    fn from(r: ULRect) -> Self {
        Rect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<ULIntRect> for Rect<i32> {
    fn from(r: ULIntRect) -> Self {
        Rect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<Rect<f32>> for ULRect {
    fn from(r: Rect<f32>) -> Self {
        ULRect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}

impl From<Rect<i32>> for ULIntRect {
    fn from(r: Rect<i32>) -> Self {
        ULIntRect {
            left: r.left,
            top: r.top,
            right: r.right,
            bottom: r.bottom,
        }
    }
}
