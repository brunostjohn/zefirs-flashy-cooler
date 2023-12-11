use crate::{
    bitmap::Bitmap,
    gpu::{
        gpu_command::GPUCommand, index_buffer::IndexBuffer, render_buffer::RenderBuffer,
        vertex_buffer::VertexBuffer,
    },
};

pub(crate) trait GPUDriver {
    fn begin_synchronize(&mut self);
    fn end_synchronize(&mut self);
    fn next_texture_id(&mut self) -> u32;
    fn create_texture(&mut self, texture_id: u32, bitmap: Bitmap);
    fn update_texture(&mut self, texture_id: u32, bitmap: Bitmap);
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
