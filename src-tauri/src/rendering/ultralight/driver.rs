use std::fs;
use std::path::PathBuf;
use std::{borrow::Cow, mem, rc::Rc};

use glium::program::{Binary, ProgramCreationInput};
use glium::vertex::AttributeType;

use kanal::Receiver;
use kanal::Sender;
use serde::{Deserialize, Serialize};

use self::gpu::GPUDriver;

use rustc_hash::FxHashMap;

use ul_sys::*;

use glium::{
    backend::{Context, Facade},
    framebuffer::SimpleFrameBuffer,
    glutin::NotCurrent,
    glutin::{platform::windows::RawContextExt, ContextBuilder, ContextWrapper},
    implement_vertex, program,
    texture::{ClientFormat, MipmapsOption, RawImage2d, SrgbTexture2d, UncompressedFloatFormat},
    uniform,
    uniforms::UniformBuffer,
    vertex::VertexBufferAny,
    Blend, DrawParameters, HeadlessRenderer, Program, Surface, Texture2d,
};

static SMALL_VERTEX: [(Cow<'static, str>, usize, i32, AttributeType, bool); 3] = [
    (
        Cow::Borrowed("in_Position"),
        0 as usize,
        -1 as i32,
        glium::vertex::AttributeType::F32F32,
        false,
    ),
    (
        Cow::Borrowed("in_Color"),
        2 * ::std::mem::size_of::<f32>() as usize,
        -1 as i32,
        glium::vertex::AttributeType::U8U8U8U8,
        true,
    ),
    (
        Cow::Borrowed("in_TexCoord"),
        2 * ::std::mem::size_of::<f32>() + 4 * ::std::mem::size_of::<u8>() as usize,
        -1 as i32,
        glium::vertex::AttributeType::F32F32,
        false,
    ),
];

static LARGE_VERTEX: [(Cow<'static, str>, usize, i32, AttributeType, bool); 11] = [
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
];

#[path = "./gpu.rs"]
pub mod gpu;
use gpu::*;

#[path = "./tex.rs"]
pub mod tex;
use tex::*;
use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;

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
                let element_size = std::mem::size_of::<ULVertex_2f_4ub_2f>();

                Ok(unsafe {
                    glium::VertexBuffer::new_raw(
                        context,
                        &buf,
                        Cow::Borrowed(SMALL_VERTEX.as_slice()),
                        element_size,
                    )
                }
                .or(Err("Failed to convert vertex"))?
                .into())
            }
            GLVertexBuffer::Format2f4ub2f2f28f(buf) => {
                let format = Cow::Borrowed(LARGE_VERTEX.as_slice());
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
    pub fn new(next_texture_id: u32, next_render_buffer_id: u32, next_geometry_id: u32) -> Self {
        let (sender, _) = kanal::unbounded();
        Self {
            next_texture_id,
            next_render_buffer_id,
            next_geometry_id,
            sender,
        }
    }

    pub fn set_tx(&mut self, sender: Sender<GPUDriverCommand>) {
        self.sender = sender;
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

                debug_assert!(head.is_empty());
                debug_assert!(tail.is_empty());

                GLVertexBuffer::Format2f4ub2f(body.to_vec())
            }

            VertexBufferFormat::Format_2f_4ub_2f_2f_28f => {
                let (head, body, tail) = unsafe {
                    vertex_buffer
                        .buffer
                        .as_slice()
                        .align_to::<ULVertex_2f_4ub_2f_2f_28f>()
                };

                debug_assert!(head.is_empty());
                debug_assert!(tail.is_empty());

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

    #[inline]
    fn create_render_buffer(&mut self, render_buffer_id: u32, render_buffer: gpu::RenderBuffer) {
        self.sender
            .send(GPUDriverCommand::CreateRenderBuffer(
                render_buffer_id,
                render_buffer,
            ))
            .unwrap();
    }

    #[inline]
    fn create_texture(&mut self, texture_id: u32, bitmap: gpu::bitmap::OwnedBitmap) {
        self.sender
            .send(GPUDriverCommand::CreateTexture(texture_id, bitmap))
            .unwrap();
    }

    #[inline]
    fn destroy_geometry(&mut self, geometry_id: u32) {
        self.sender
            .send(GPUDriverCommand::DestroyGeometry(geometry_id))
            .unwrap();
    }

    #[inline]
    fn destroy_render_buffer(&mut self, render_buffer_id: u32) {
        self.sender
            .send(GPUDriverCommand::DestroyRenderBuffer(render_buffer_id))
            .unwrap();
    }

    #[inline]
    fn destroy_texture(&mut self, texture_id: u32) {
        self.sender
            .send(GPUDriverCommand::DestroyTexture(texture_id))
            .unwrap();
    }

    #[inline]
    fn end_synchronize(&mut self) {}

    #[inline]
    fn next_geometry_id(&mut self) -> u32 {
        self.next_geometry_id += 1;

        self.next_geometry_id
    }

    #[inline]
    fn next_render_buffer_id(&mut self) -> u32 {
        self.next_render_buffer_id += 1;

        self.next_render_buffer_id
    }

    #[inline]
    fn next_texture_id(&mut self) -> u32 {
        self.next_texture_id += 1;

        self.next_texture_id
    }

    #[inline]
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

                debug_assert!(head.is_empty());
                debug_assert!(tail.is_empty());

                GLVertexBuffer::Format2f4ub2f(body.to_vec())
            }

            VertexBufferFormat::Format_2f_4ub_2f_2f_28f => {
                let (head, body, tail) = unsafe {
                    vertex_buffer
                        .buffer
                        .as_slice()
                        .align_to::<ULVertex_2f_4ub_2f_2f_28f>()
                };

                debug_assert!(head.is_empty());
                debug_assert!(tail.is_empty());

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

    #[inline]
    fn update_texture(&mut self, texture_id: u32, bitmap: OwnedBitmap) {
        self.sender
            .send(GPUDriverCommand::UpdateTexture(texture_id, bitmap))
            .unwrap();
    }
}

#[derive(Copy, Clone)]
struct RenderVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(RenderVertex, position, tex_coords);

#[allow(dead_code)]
pub struct GPUDriverReceiver {
    receiver: Receiver<GPUDriverCommand>,
    context: Rc<Context>,
    head: HeadlessRenderer,
    texture_map: FxHashMap<u32, (EitherTexture, Option<u32>)>,
    empty_texture: EitherTexture,
    render_buffer_map: FxHashMap<u32, RenderBuffer>,
    geometry_map: FxHashMap<u32, (VertexBufferAny, glium::IndexBuffer<u32>)>,
    path_program: Program,
    fill_program: Program,
    rawdog: ContextWrapper<NotCurrent, ()>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShaderCacheFormat {
    pub fill_format: u32,
    pub path_format: u32,
}

impl GPUDriverReceiver {
    pub fn new(
        receiver: Receiver<GPUDriverCommand>,
        app_folder: PathBuf,
    ) -> Result<Self, &'static str> {
        let ctx = unsafe { ContextBuilder::new().build_raw_context(GetDesktopWindow().0) }.unwrap();

        let gl_ctx = HeadlessRenderer::with_debug::<NotCurrent>(
            unsafe { mem::transmute_copy(ctx.context()) },
            glium::debug::DebugCallbackBehavior::PrintAll,
        )
        .unwrap();

        let context = gl_ctx.get_context().clone();

        let empty_texture = EitherTexture::Regular2d(
            Texture2d::empty(&context, 1, 1)
                .or(Err("Failed to create empty texture!"))
                .unwrap(),
        );

        let texture_map = FxHashMap::default();
        let render_buffer_map = FxHashMap::default();
        let geometry_map = FxHashMap::default();

        let path_program;
        let fill_program;

        let shader_cache = {
            let mut app_folder: PathBuf = app_folder.clone();
            app_folder.push("shader_cache");
            app_folder
        };

        let mut shader_cache_vers = {
            let mut shaderc = shader_cache.clone();
            shaderc.push("shader_binaries.json");

            let vers = fs::read_to_string(shaderc)
                .or::<Result<String, &'static str>>(Ok("".to_string()))
                .unwrap();

            let bins: ShaderCacheFormat = serde_json::from_str(&vers)
                .or::<Result<ShaderCacheFormat, &'static str>>(Ok(ShaderCacheFormat {
                    path_format: 0,
                    fill_format: 0,
                }))
                .unwrap();

            bins
        };

        if !shader_cache.exists() {
            fs::create_dir(&shader_cache).unwrap();
        }

        let path_shader_location = {
            let mut folder = shader_cache.clone();
            folder.push("path.cached");
            folder
        };

        if path_shader_location.exists() && shader_cache_vers.path_format != 0 {
            let bin = fs::read(path_shader_location).unwrap();
            let format = shader_cache_vers.path_format;

            let binary = Binary {
                format,
                content: bin,
            };

            let creation_input = ProgramCreationInput::from(binary);

            path_program = Program::new(&context, creation_input).unwrap();
        } else {
            path_program = program!(&context,
            150 => {
                vertex: include_str!("./shaders/v2f_c4f_t2f_vert.vert"),
                fragment: include_str!("./shaders/path_frag.frag")
            })
            .unwrap();

            let _ = path_program.get_binary().and_then(|binary| {
                shader_cache_vers.path_format = binary.format;

                let _ = fs::write(path_shader_location, binary.content);

                Ok(())
            });
        }

        let fill_shader_location = {
            let mut folder = shader_cache.clone();
            folder.push("fill.cached");
            folder
        };

        if fill_shader_location.exists() && shader_cache_vers.fill_format != 0 {
            let bin = fs::read(fill_shader_location).unwrap();
            let format = shader_cache_vers.fill_format;

            let binary = Binary {
                format,
                content: bin,
            };

            let creation_input = ProgramCreationInput::from(binary);

            fill_program = Program::new(&context, creation_input).unwrap();
        } else {
            fill_program = program!(&context,
            150 => {
                vertex: include_str!("./shaders/v2f_c4f_t2f_t2f_d28f_vert.vert"),
                fragment: include_str!("./shaders/fill_frag.frag")
            })
            .or(Err("Failed to create fill shader!"))
            .unwrap();

            let _ = fill_program.get_binary().and_then(|binary| {
                shader_cache_vers.fill_format = binary.format;

                let _ = fs::write(fill_shader_location, binary.content);

                Ok(())
            });
        }

        let updated_vers = serde_json::to_string(&shader_cache_vers).unwrap();

        let _ = fs::write(
            {
                let mut sc = shader_cache;
                sc.push("shader_binaries.json");
                sc
            },
            updated_vers,
        );

        Ok(Self {
            receiver,
            context: context.get_context().clone(),
            empty_texture,
            texture_map,
            render_buffer_map,
            geometry_map,

            path_program,
            fill_program,
            head: gl_ctx,
            rawdog: ctx,
        })
    }

    #[inline]
    pub fn render_bitmap(&mut self, tex_id: u32) -> Result<Cow<'_, [u8]>, &'static str> {
        let image = self.get_texture(&tex_id).unwrap();

        let image_data: RawImage2d<'_, u8> = image.data().read_as_texture_2d().unwrap();

        let image = image_data.data;

        Ok(image)
    }

    #[inline]
    pub fn get_texture(&self, id: &u32) -> Option<&EitherTexture> {
        self.texture_map.get(id).map(|(t, _)| t)
    }

    #[inline]
    pub fn render(&mut self) -> Result<(), &'static str> {
        while let Some(cmd) = self.receiver.try_recv().or(Err("Failed to receive"))? {
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
        #[allow(unused_variables)] id: u32,
        bitmap: OwnedBitmap,
    ) -> Result<EitherTexture, &'static str> {
        let tex;

        if bitmap.is_empty() {
            tex = Texture2d::empty(&self.context, bitmap.width(), bitmap.height())
                .map_err(|_| "Failed to create texture")
                .map(|t| EitherTexture::Regular2d(t))?;
        } else {
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
        debug_assert!(self.texture_map.contains_key(&id));

        let tex = self.create_texture(id, bitmap)?;

        (*self
            .texture_map
            .get_mut(&id)
            .expect("Failed to get updated texture"))
        .0 = tex;

        Ok(())
    }

    #[inline]
    fn destroy_texture(&mut self, id: u32) -> Result<(), &'static str> {
        debug_assert!(self.texture_map.contains_key(&id));

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

        debug_assert!(tex.0.width() == render_buffer.width);
        debug_assert!(tex.0.height() == render_buffer.height);

        self.render_buffer_map.insert(id, render_buffer);

        Ok(())
    }

    fn destroy_render_buffer(&mut self, id: u32) -> Result<(), &'static str> {
        debug_assert!(self.render_buffer_map.contains_key(&id));

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
        debug_assert!(self.geometry_map.contains_key(&id));

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

    #[inline]
    fn destroy_geometry(&mut self, id: u32) -> Result<(), &'static str> {
        debug_assert!(self.geometry_map.contains_key(&id));

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
        debug_assert!(self.render_buffer_map.contains_key(&render_buffer_id));

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
        debug_assert!(self.geometry_map.contains_key(&geometry_id));

        let (vertex_buffer, index_buffer) = self
            .geometry_map
            .get(&geometry_id)
            .expect("Failed to get vert, i for draw");

        debug_assert!(self
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
