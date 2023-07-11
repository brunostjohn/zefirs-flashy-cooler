use std::{
    borrow::Cow,
    collections::HashMap,
    rc::Rc,
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
};

use self::gpu::GPUDriver;

use ul_sys::*;

use glium::{
    backend::{Context, Facade},
    framebuffer::SimpleFrameBuffer,
    program,
    texture::{ClientFormat, MipmapsOption, RawImage2d, SrgbTexture2d, UncompressedFloatFormat},
    uniform,
    uniforms::UniformBuffer,
    vertex::VertexBufferAny,
    Blend, DrawParameters, Program, Surface, Texture2d,
};

#[path = "./gpu.rs"]
pub mod gpu;
use gpu::*;

#[path = "./tex.rs"]
pub mod tex;
use tex::*;

pub enum GLVertexBuffer {
    Format2f4ub2f(Vec<ULVertex_2f_4ub_2f>),
    Format2f4ub2f2f28f(Vec<ULVertex_2f_4ub_2f_2f_28f>),
}

impl GLVertexBuffer {
    fn into_vertex_buffer<F: ?Sized>(self, context: &F) -> Result<VertexBufferAny, &'static str>
    where
        F: Facade,
    {
        match self {
            GLVertexBuffer::Format2f4ub2f(buf) => {
                let format = Cow::Owned(vec![
                    (
                        Cow::Borrowed("in_Position"),
                        0,
                        -1,
                        glium::vertex::AttributeType::F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Color"),
                        2 * ::std::mem::size_of::<f32>(),
                        -1,
                        glium::vertex::AttributeType::U8U8U8U8,
                        true,
                    ),
                    (
                        Cow::Borrowed("in_TexCoord"),
                        2 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32,
                        false,
                    ),
                ]);
                let element_size = std::mem::size_of::<ULVertex_2f_4ub_2f>();

                Ok(
                    unsafe { glium::VertexBuffer::new_raw(context, &buf, format, element_size) }
                        .or(Err("Failed to convert vertex"))?
                        .into(),
                )
            }
            GLVertexBuffer::Format2f4ub2f2f28f(buf) => {
                let format = Cow::Owned(vec![
                    (
                        Cow::Borrowed("in_Position"),
                        0,
                        -1,
                        glium::vertex::AttributeType::F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Color"),
                        2 * ::std::mem::size_of::<f32>(),
                        -1,
                        glium::vertex::AttributeType::U8U8U8U8,
                        true,
                    ),
                    (
                        Cow::Borrowed("in_TexCoord"),
                        2 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_ObjCoord"),
                        4 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Data0"),
                        6 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Data1"),
                        10 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Data2"),
                        14 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Data3"),
                        18 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Data4"),
                        22 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Data5"),
                        26 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32F32F32,
                        false,
                    ),
                    (
                        Cow::Borrowed("in_Data6"),
                        30 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>(),
                        -1,
                        glium::vertex::AttributeType::F32F32F32F32,
                        false,
                    ),
                ]);
                let element_size = std::mem::size_of::<ULVertex_2f_4ub_2f_2f_28f>();

                Ok(
                    unsafe { glium::VertexBuffer::new_raw(context, &buf, format, element_size) }
                        .or(Err("Failed to convert vertex buffer."))?
                        .into(),
                )
            }
        }
    }
}

pub enum GPUDriverCommand {
    CreateTexture(u32, OwnedBitmap),
    UpdateTexture(u32, OwnedBitmap),
    DestroyTexture(u32),
    CreateRenderBuffer(u32, RenderBuffer),
    DestroyRenderBuffer(u32),
    CreateGeometry(u32, GLVertexBuffer, IndexBuffer),
    UpdateGeometry(u32, GLVertexBuffer, IndexBuffer),
    DestroyGeometry(u32),
    UpdateCommandList(Vec<GPUCommand>),
}

pub struct GPUDriverSender {
    next_texture_id: u32,
    next_render_buffer_id: u32,
    next_geometry_id: u32,
    sender: Sender<GPUDriverCommand>,
}

impl GPUDriverSender {
    pub fn new(
        next_texture_id: u32,
        next_render_buffer_id: u32,
        next_geometry_id: u32,
        sender: Sender<GPUDriverCommand>,
    ) -> Self {
        Self {
            next_texture_id,
            next_render_buffer_id,
            next_geometry_id,
            sender,
        }
    }
}

impl GPUDriver for GPUDriverSender {
    fn begin_synchronize(&mut self) {}

    fn create_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: gpu::VertexBuffer,
        index_buffer: gpu::IndexBuffer,
    ) {
        let gl_vertex_buffer = match vertex_buffer.format {
            VertexBufferFormat::Format_2f_4ub_2f => {
                let (head, body, tail) = unsafe {
                    vertex_buffer
                        .buffer
                        .as_slice()
                        .align_to::<ULVertex_2f_4ub_2f>()
                };

                assert!(head.is_empty());
                assert!(tail.is_empty());

                GLVertexBuffer::Format2f4ub2f(body.to_vec())
            }

            VertexBufferFormat::Format_2f_4ub_2f_2f_28f => {
                let (head, body, tail) = unsafe {
                    vertex_buffer
                        .buffer
                        .as_slice()
                        .align_to::<ULVertex_2f_4ub_2f_2f_28f>()
                };

                assert!(head.is_empty());
                assert!(tail.is_empty());

                GLVertexBuffer::Format2f4ub2f2f28f(body.to_vec())
            }
        };

        self.sender
            .send(GPUDriverCommand::CreateGeometry(
                geometry_id,
                gl_vertex_buffer,
                index_buffer,
            ))
            .unwrap();
    }

    fn create_render_buffer(&mut self, render_buffer_id: u32, render_buffer: gpu::RenderBuffer) {
        self.sender
            .send(GPUDriverCommand::CreateRenderBuffer(
                render_buffer_id,
                render_buffer,
            ))
            .unwrap();
    }

    fn create_texture(&mut self, texture_id: u32, bitmap: gpu::bitmap::OwnedBitmap) {
        self.sender
            .send(GPUDriverCommand::CreateTexture(texture_id, bitmap))
            .unwrap();
    }

    fn destroy_geometry(&mut self, geometry_id: u32) {
        self.sender
            .send(GPUDriverCommand::DestroyGeometry(geometry_id))
            .unwrap();
    }

    fn destroy_render_buffer(&mut self, render_buffer_id: u32) {
        self.sender
            .send(GPUDriverCommand::DestroyRenderBuffer(render_buffer_id))
            .unwrap();
    }

    fn destroy_texture(&mut self, texture_id: u32) {
        self.sender
            .send(GPUDriverCommand::DestroyTexture(texture_id))
            .unwrap();
    }

    fn end_synchronize(&mut self) {}

    fn next_geometry_id(&mut self) -> u32 {
        self.next_geometry_id += 1;

        self.next_geometry_id
    }

    fn next_render_buffer_id(&mut self) -> u32 {
        self.next_render_buffer_id += 1;

        self.next_render_buffer_id
    }

    fn next_texture_id(&mut self) -> u32 {
        self.next_texture_id += 1;

        self.next_texture_id
    }

    fn update_command_list(&mut self, command_list: Vec<gpu::GPUCommand>) {
        self.sender
            .send(GPUDriverCommand::UpdateCommandList(command_list))
            .unwrap();
    }

    fn update_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: gpu::VertexBuffer,
        index_buffer: gpu::IndexBuffer,
    ) {
        let gl_vertex_buffer = match vertex_buffer.format {
            VertexBufferFormat::Format_2f_4ub_2f => {
                let (head, body, tail) = unsafe {
                    vertex_buffer
                        .buffer
                        .as_slice()
                        .align_to::<ULVertex_2f_4ub_2f>()
                };

                assert!(head.is_empty());
                assert!(tail.is_empty());

                GLVertexBuffer::Format2f4ub2f(body.to_vec())
            }

            VertexBufferFormat::Format_2f_4ub_2f_2f_28f => {
                let (head, body, tail) = unsafe {
                    vertex_buffer
                        .buffer
                        .as_slice()
                        .align_to::<ULVertex_2f_4ub_2f_2f_28f>()
                };

                assert!(head.is_empty());
                assert!(tail.is_empty());

                GLVertexBuffer::Format2f4ub2f2f28f(body.to_vec())
            }
        };

        self.sender
            .send(GPUDriverCommand::UpdateGeometry(
                geometry_id,
                gl_vertex_buffer,
                index_buffer,
            ))
            .unwrap();
    }

    fn update_texture(&mut self, texture_id: u32, bitmap: OwnedBitmap) {
        self.sender
            .send(GPUDriverCommand::UpdateTexture(texture_id, bitmap))
            .unwrap();
    }
}

pub struct GPUDriverReceiver {
    receiver: Receiver<GPUDriverCommand>,
    context: Rc<Context>,
    texture_map: HashMap<u32, (EitherTexture, Option<u32>)>,
    empty_texture: EitherTexture,
    render_buffer_map: HashMap<u32, RenderBuffer>,
    geometry_map: HashMap<u32, (VertexBufferAny, glium::IndexBuffer<u32>)>,
    path_program: Program,
    fill_program: Program,
}

impl GPUDriverReceiver {
    pub fn new(
        receiver: Receiver<GPUDriverCommand>,
        context: &Rc<Context>,
    ) -> Result<Self, &'static str> {
        let context = context.clone();
        let empty_texture = EitherTexture::Regular2d(
            Texture2d::empty(&context, 1, 1).or(Err("Failed to create empty texture!"))?,
        );

        let texture_map = HashMap::new();
        let render_buffer_map = HashMap::new();
        let geometry_map = HashMap::new();

        let path_program = program!(&context,
        150 => {
            vertex: include_str!("./shaders/v2f_c4f_t2f_vert.glsl"),
            fragment: include_str!("./shaders/path_frag.glsl")
        })
        .or(Err("Failed to create path shader!"))?;

        let fill_program = program!(&context,
        150 => {
            vertex: include_str!("./shaders/v2f_c4f_t2f_t2f_d28f_vert.glsl"),
            fragment: include_str!("./shaders/fill_frag.glsl")
        })
        .or(Err("Failed to create fill shader!"))?;

        Ok(Self {
            receiver,
            context,
            empty_texture,
            texture_map,
            render_buffer_map,
            geometry_map,

            path_program,
            fill_program,
        })
    }

    pub fn render(&mut self) -> Result<(), &'static str> {
        while let Ok(cmd) = self.receiver.try_recv() {
            match cmd {
                GPUDriverCommand::CreateTexture(id, bitmap) => {
                    let tex = self.create_texture(id, bitmap)?;

                    self.texture_map.insert(id, (tex, None));
                }
                GPUDriverCommand::UpdateTexture(id, bitmap) => {
                    self.update_texture(id, bitmap)?;
                }
                GPUDriverCommand::DestroyTexture(id) => {
                    self.destroy_texture(id)?;
                }
                GPUDriverCommand::CreateRenderBuffer(id, render_buffer) => {
                    self.create_render_buffer(id, render_buffer)?;
                }
                GPUDriverCommand::DestroyRenderBuffer(id) => {
                    self.destroy_render_buffer(id)?;
                }
                GPUDriverCommand::CreateGeometry(id, vertex, index) => {
                    self.create_geometry(id, vertex, index)?;
                }
                GPUDriverCommand::UpdateGeometry(id, vertex, index) => {
                    self.update_geometry(id, vertex, index)?;
                }
                GPUDriverCommand::DestroyGeometry(id) => {
                    self.destroy_geometry(id)?;
                }
                GPUDriverCommand::UpdateCommandList(command_list) => {
                    self.update_command_list(command_list)?;
                }
            }
        }

        Ok(())
    }

    fn create_texture(
        &mut self,
        id: u32,
        bitmap: OwnedBitmap,
    ) -> Result<EitherTexture, &'static str> {
        let tex;

        if bitmap.is_empty() {
            tex = Texture2d::empty(&self.context, bitmap.width(), bitmap.height())
                .map_err(|_| "Failed to create texture")
                .map(|t| EitherTexture::Regular2d(t))?;
        } else {
            // since its not empty, it should have a valid pixels.
            let bitmap_pixels = bitmap.pixels().unwrap();
            match bitmap.format() {
                BitmapFormat::A8Unorm => {
                    let img = RawImage2d {
                        data: Cow::Borrowed(bitmap_pixels),
                        width: bitmap.width(),
                        height: bitmap.height(),
                        format: ClientFormat::U8,
                    };

                    tex = Texture2d::with_format(
                        &self.context,
                        img,
                        UncompressedFloatFormat::U8,
                        MipmapsOption::NoMipmap,
                    )
                    .map_err(|_| "Failed to create texture")
                    .map(|t| EitherTexture::Regular2d(t))?;
                }
                BitmapFormat::Bgra8UnormSrgb => {
                    let img = RawImage2d {
                        data: Cow::Borrowed(bitmap_pixels),
                        width: bitmap.width(),
                        height: bitmap.height(),
                        format: ClientFormat::U8U8U8U8,
                    };

                    tex = SrgbTexture2d::with_format(
                        &self.context,
                        img,
                        glium::texture::SrgbFormat::U8U8U8U8,
                        MipmapsOption::NoMipmap,
                    )
                    .map_err(|_| "Failed to create texture")
                    .map(|t| EitherTexture::Srgb2d(t))?;
                }
            }
        }

        Ok(tex)
    }

    fn update_texture(&mut self, id: u32, bitmap: OwnedBitmap) -> Result<(), &'static str> {
        assert!(self.texture_map.contains_key(&id));

        let tex = self.create_texture(id, bitmap)?;

        (*self
            .texture_map
            .get_mut(&id)
            .expect("Failed to get updated texture"))
        .0 = tex;

        Ok(())
    }

    fn destroy_texture(&mut self, id: u32) -> Result<(), &'static str> {
        assert!(self.texture_map.contains_key(&id));

        self.texture_map.remove(&id);

        Ok(())
    }

    fn create_render_buffer(
        &mut self,
        id: u32,
        render_buffer: RenderBuffer,
    ) -> Result<(), &'static str> {
        let tex = self
            .texture_map
            .get_mut(&render_buffer.texture_id)
            .expect("No tex with id of render_buffer found!");

        tex.1 = Some(id);

        assert!(tex.0.width() == render_buffer.width);
        assert!(tex.0.height() == render_buffer.height);

        self.render_buffer_map.insert(id, render_buffer);

        Ok(())
    }

    fn destroy_render_buffer(&mut self, id: u32) -> Result<(), &'static str> {
        assert!(self.render_buffer_map.contains_key(&id));

        let render_buffer = self
            .render_buffer_map
            .remove(&id)
            .expect("Failed to fetch render buffer!");

        if let Some(tex) = self.texture_map.get_mut(&render_buffer.texture_id) {
            tex.1 = None;
        }

        Ok(())
    }

    fn create_geometry(
        &mut self,
        id: u32,
        vertex: GLVertexBuffer,
        index: IndexBuffer,
    ) -> Result<(), &'static str> {
        let index_buffer = glium::IndexBuffer::new(
            &self.context,
            glium::index::PrimitiveType::TrianglesList,
            &index.buffer,
        )
        .or(Err("Failed to create indexbuffer"))?;

        self.geometry_map.insert(
            id,
            (vertex.into_vertex_buffer(&self.context)?, index_buffer),
        );

        Ok(())
    }

    fn update_geometry(
        &mut self,
        id: u32,
        vertex: GLVertexBuffer,
        index: IndexBuffer,
    ) -> Result<(), &'static str> {
        assert!(self.geometry_map.contains_key(&id));

        let index_buffer = glium::IndexBuffer::new(
            &self.context,
            glium::index::PrimitiveType::TrianglesList,
            &index.buffer,
        )
        .or(Err("Failed to update geometry"))?;

        *self
            .geometry_map
            .get_mut(&id)
            .expect("Failed to get geometry from map") =
            (vertex.into_vertex_buffer(&self.context)?, index_buffer);

        Ok(())
    }

    fn destroy_geometry(&mut self, id: u32) -> Result<(), &'static str> {
        assert!(self.geometry_map.contains_key(&id));

        self.geometry_map.remove(&id);

        Ok(())
    }

    fn update_command_list(&mut self, command_list: Vec<GPUCommand>) -> Result<(), &'static str> {
        for cmd in command_list {
            match cmd {
                GPUCommand::ClearRenderBuffer { render_buffer_id } => {
                    self.clear_render_buffer(render_buffer_id)?;
                }
                GPUCommand::DrawGeometry {
                    gpu_state,
                    geometry_id,
                    indices_offset,
                    indices_count,
                } => {
                    self.draw_geometry(gpu_state, geometry_id, indices_offset, indices_count)?;
                }
            }
        }

        Ok(())
    }

    fn clear_render_buffer(&mut self, render_buffer_id: u32) -> Result<(), &'static str> {
        assert!(self.render_buffer_map.contains_key(&render_buffer_id));

        let render_buffer = self
            .render_buffer_map
            .get(&render_buffer_id)
            .expect("Failed to fetch render buffer for clearing");

        let tex = self
            .texture_map
            .get(&render_buffer.texture_id)
            .expect("Failed to get tex for clear");

        let mut frame_buffer = SimpleFrameBuffer::new(&self.context, &tex.0)
            .or(Err("Failed to create framebuffer"))?;

        frame_buffer.clear(None, Some((0.0, 0.0, 0.0, 0.0)), false, None, None);

        Ok(())
    }

    fn draw_geometry(
        &mut self,
        gpu_state: Box<GPUState>,
        geometry_id: u32,
        indices_offset: u32,
        indices_count: u32,
    ) -> Result<(), &'static str> {
        assert!(self.geometry_map.contains_key(&geometry_id));

        let (vertex_buffer, index_buffer) = self
            .geometry_map
            .get(&geometry_id)
            .expect("Failed to get vert, i for draw");

        assert!(self
            .render_buffer_map
            .contains_key(&gpu_state.render_buffer_id));

        let render_buffer = self
            .render_buffer_map
            .get(&gpu_state.render_buffer_id)
            .expect("Failed to fetch render buf for draw");

        let index_buffer_slice = index_buffer
            .slice(indices_offset as usize..(indices_offset as usize + indices_count as usize))
            .ok_or("Failed to get slice of i buf - draw")?;

        let (t, _) = self.texture_map.get(&render_buffer.texture_id).unwrap();

        let mut frame_buffer =
            SimpleFrameBuffer::new(&self.context, t).or(Err("Failed to create draw frame buf"))?;

        let used_shader = match gpu_state.shader_type {
            ShaderType::Fill => &self.fill_program,
            ShaderType::FillPath => &self.path_program,
        };

        let scalar_data = UniformBuffer::new(&self.context, gpu_state.uniform_scalar)
            .or(Err("Failed to create scalar"))?;
        let vector_data = UniformBuffer::new(&self.context, gpu_state.uniform_vector)
            .or(Err("Failed to create vecdata"))?;
        let clip_data = UniformBuffer::new(&self.context, gpu_state.clip)
            .or(Err("Failed to create clipdata"))?;

        let orth_projection_matrix = [
            [2.0 / gpu_state.viewport_width as f32, 0.0, 0.0, 0.0],
            [0.0, 2.0 / gpu_state.viewport_height as f32, 0.0, 0.0],
            [0.0, 0.0, -0.000002, 0.0],
            [-1.0, -1.0, 0.818183, 1.0],
        ];

        let mut transformation = [
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
            [0., 0., 0., 0.],
        ];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    transformation[i][j] +=
                        gpu_state.transform[i * 4 + k] * orth_projection_matrix[k][j];
                }
            }
        }

        let texture1 = if let Some(id) = gpu_state.texture_1_id {
            let (t, _) = self.texture_map.get(&id).unwrap();
            t
        } else {
            &self.empty_texture
        };
        let texture2 = if let Some(id) = gpu_state.texture_2_id {
            let (t, _) = self.texture_map.get(&id).unwrap();
            t
        } else {
            &self.empty_texture
        };
        let texture3 = if let Some(id) = gpu_state.texture_3_id {
            let (t, _) = self.texture_map.get(&id).unwrap();
            t
        } else {
            &self.empty_texture
        };

        let uniforms = uniform! {
            State: [0.0, gpu_state.viewport_width as f32, gpu_state.viewport_height as f32, 1.0],
            Transform: transformation,
            Scalar: &scalar_data,
            Vector: &vector_data,
            ClipSize: gpu_state.clip_size,
            Clip: &clip_data,
            Texture1: texture1.sampled(),
            Texture2: texture2.sampled(),
            Texture3: texture3.sampled(),
        };

        let params = DrawParameters {
            viewport: Some(glium::Rect {
                left: 0,
                bottom: 0,
                width: gpu_state.viewport_width,
                height: gpu_state.viewport_height,
            }),
            scissor: if gpu_state.enable_scissor {
                Some(glium::Rect {
                    left: gpu_state.scissor_rect.left as u32,
                    bottom: gpu_state.scissor_rect.top as u32,
                    width: (gpu_state.scissor_rect.right - gpu_state.scissor_rect.left) as u32,
                    height: (gpu_state.scissor_rect.bottom - gpu_state.scissor_rect.top) as u32,
                })
            } else {
                None
            },
            blend: if gpu_state.enable_blend {
                Blend::alpha_blending()
            } else {
                Blend::default()
            },
            ..DrawParameters::default()
        };

        frame_buffer
            .draw(
                vertex_buffer,
                index_buffer_slice,
                used_shader,
                &uniforms,
                &params,
            )
            .or(Err("Failed to draw"))?;

        Ok(())
    }
}
