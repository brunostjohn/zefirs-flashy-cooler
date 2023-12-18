use ultralight_sys::{ULVertex_2f_4ub_2f, ULVertex_2f_4ub_2f_2f_28f};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages,
};
use wgpu_async::AsyncDevice;

use crate::bitmap::Bitmap;

use super::{gpu_command::GPUCommand, index_buffer::IndexBuffer, render_buffer::RenderBuffer};

#[derive(Debug)]
pub enum GPUDriverCommand {
    CreateTexture(u32, Bitmap),
    UpdateTexture(u32, Bitmap),
    DestroyTexture(u32),
    CreateRenderBuffer(u32, RenderBuffer),
    DestroyRenderBuffer(u32),
    CreateGeometry(u32, DriverVertexBuffer, IndexBuffer),
    UpdateGeometry(u32, DriverVertexBuffer, IndexBuffer),
    DestroyGeometry(u32),
    UpdateCommandList(Vec<GPUCommand>),
}

#[derive(Debug)]
pub enum DriverVertexBuffer {
    Format2f4ub2f(Vec<ULVertex_2f_4ub_2f>),
    Format2f4ub2f2f28f(Vec<ULVertex_2f_4ub_2f_2f_28f>),
}

impl DriverVertexBuffer {
    pub(crate) fn size(&self) -> usize {
        match self {
            DriverVertexBuffer::Format2f4ub2f(v) => {
                v.len() * std::mem::size_of::<ULVertex_2f_4ub_2f>()
            }
            DriverVertexBuffer::Format2f4ub2f2f28f(v) => {
                v.len() * std::mem::size_of::<ULVertex_2f_4ub_2f_2f_28f>()
            }
        }
    }

    pub(crate) fn into_gpu_buffer(self, device: &AsyncDevice) -> Buffer {
        match self {
            DriverVertexBuffer::Format2f4ub2f(v) => {
                let (head, body, _tail) = unsafe { v.align_to::<u8>() };
                assert!(head.is_empty(), "ULVertex_2f_4ub_2f is not aligned");
                device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("vertex_buffer 2f_4ub_2f"),
                    usage: BufferUsages::VERTEX,
                    contents: body,
                })
            }
            DriverVertexBuffer::Format2f4ub2f2f28f(v) => {
                let (head, body, _tail) = unsafe { v.align_to::<u8>() };
                assert!(head.is_empty(), "ULVertex_2f_4ub_2f_2f_28f is not aligned");
                device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("vertex_buffer 2f_4ub_2f_2f_28f"),
                    usage: BufferUsages::VERTEX,
                    contents: body,
                })
            }
        }
    }
}
