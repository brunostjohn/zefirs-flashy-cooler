use std::{borrow::Cow, collections::HashMap, path::Path, sync::Arc};

use nohash_hasher::BuildNoHashHasher;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, Buffer, BufferUsages, CommandEncoderDescriptor, DeviceDescriptor, Extent3d, Features,
    Instance, InstanceDescriptor, Limits, LoadOp, Operations, PowerPreference,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, ShaderModule, StoreOp,
    Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};
use wgpu_async::{AsyncDevice, AsyncQueue};

use crate::{
    bitmap::{Bitmap, BitmapFormat},
    error::ULError,
    gpu::{
        gpu_command::GPUCommand, gpu_state::GPUState, index_buffer::IndexBuffer,
        shader_type::ShaderType, worker_command::DriverVertexBuffer,
    },
    types::gpu::render_buffer::RenderBuffer,
    ULResult,
};

use super::{
    driver_trait::GPUDriver,
    shaders::{get_fill_path_shader, get_fill_shader},
};

pub struct GPUDriverReceiver {
    instance: Instance,
    adapter: Adapter,
    device: AsyncDevice,
    queue: AsyncQueue,
    texture_map: HashMap<u32, (Texture, Option<u32>), BuildNoHashHasher<u32>>,
    render_buffer_map: HashMap<u32, RenderBuffer, BuildNoHashHasher<u32>>,
    geometry_map: HashMap<u32, (Buffer, Buffer), BuildNoHashHasher<u32>>,
    fill_shader_module: ShaderModule,
    path_shader_module: ShaderModule,
    v2f_c4f_t2f_shader_module: ShaderModule,
    v2f_c4f_t2f_d28f_shader_module: ShaderModule,
    empty_texture: Texture,
}

impl GPUDriverReceiver {
    async fn init_gpu() -> ULResult<(Instance, Adapter, AsyncDevice, AsyncQueue)> {
        let instance = Instance::new(InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::LowPower,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or(ULError::GPUDriverNoCompatibleAdapter)?;
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Ultralight Device"),
                    features: Features::default(),
                    limits: Limits::default(),
                },
                None,
            )
            .await
            .or(Err(ULError::GPUDriverNoCompatibleDevice))?;
        let (device, queue) = wgpu_async::wrap(Arc::new(device), Arc::new(queue));
        Ok((instance, adapter, device, queue))
    }

    fn create_hash_maps() -> (
        HashMap<u32, (Texture, Option<u32>), BuildNoHashHasher<u32>>,
        HashMap<u32, RenderBuffer, BuildNoHashHasher<u32>>,
        HashMap<u32, (Buffer, Buffer), BuildNoHashHasher<u32>>,
    ) {
        (
            HashMap::with_capacity_and_hasher(10, BuildNoHashHasher::default()),
            HashMap::with_capacity_and_hasher(10, BuildNoHashHasher::default()),
            HashMap::with_capacity_and_hasher(10, BuildNoHashHasher::default()),
        )
    }

    fn init_shaders(
        device: &AsyncDevice,
    ) -> (ShaderModule, ShaderModule, ShaderModule, ShaderModule) {
        (
            device.create_shader_module(get_fill_shader()),
            device.create_shader_module(get_fill_path_shader()),
            device.create_shader_module(get_fill_path_shader()),
            device.create_shader_module(get_fill_path_shader()),
        )
    }

    fn create_empty_texture(device: &AsyncDevice, width: u32, height: u32) -> Texture {
        let texture_descriptor = TextureDescriptor {
            label: Some("Empty Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[TextureFormat::Rgba8UnormSrgb],
        };

        let texture = device.create_texture(&texture_descriptor);

        texture
    }

    pub(crate) async fn new<P: AsRef<Path>>(shader_cache_path: P) -> ULResult<Self> {
        let (instance, adapter, device, queue) = Self::init_gpu().await?;

        let (texture_map, render_buffer_map, geometry_map) = Self::create_hash_maps();

        let (
            fill_shader_module,
            path_shader_module,
            v2f_c4f_t2f_shader_module,
            v2f_c4f_t2f_d28f_shader_module,
        ) = Self::init_shaders(&device);

        let empty_texture = Self::create_empty_texture(&device, 1, 1);

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            texture_map,
            render_buffer_map,
            geometry_map,
            fill_shader_module,
            path_shader_module,
            v2f_c4f_t2f_shader_module,
            v2f_c4f_t2f_d28f_shader_module,
            empty_texture,
        })
    }

    pub fn create_texture(
        &mut self,
        #[allow(unused_variables)] id: u32,
        bitmap: Bitmap,
    ) -> ULResult<Texture> {
        if bitmap.is_empty() {
            Ok(Self::create_empty_texture(&self.device, 1, 1))
        } else {
            let pixels = bitmap.pixels()?;

            match bitmap.format()? {
                BitmapFormat::A8Unorm => {
                    let texture = self.device.create_texture_with_data(
                        &self.queue,
                        &TextureDescriptor {
                            size: Extent3d {
                                width: bitmap.width(),
                                height: bitmap.height(),
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: TextureDimension::D2,
                            format: TextureFormat::R8Unorm,
                            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                            label: Some("Ultralight Texture"),
                            view_formats: &[],
                        },
                        pixels.as_slice(),
                    );

                    Ok(texture)
                }
                BitmapFormat::Bgra8UnormSrgb => {
                    let texture = self.device.create_texture_with_data(
                        &self.queue,
                        &TextureDescriptor {
                            size: Extent3d {
                                width: bitmap.width(),
                                height: bitmap.height(),
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: TextureDimension::D2,
                            format: TextureFormat::Bgra8UnormSrgb,
                            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                            label: Some("Ultralight Texture"),
                            view_formats: &[],
                        },
                        pixels.as_slice(),
                    );

                    Ok(texture)
                }
            }
        }
    }

    pub fn get_texture(&self, id: &u32) -> Option<&Texture> {
        self.texture_map.get(id).map(|(t, _)| t)
    }

    fn update_texture(&mut self, id: u32, bitmap: Bitmap) -> ULResult<()> {
        debug_assert!(self.texture_map.contains_key(&id));

        let tex = self.create_texture(id, bitmap)?;

        self.texture_map
            .get_mut(&id)
            .expect("Failed to get updated texture")
            .0 = tex;

        Ok(())
    }

    fn destroy_texture(&mut self, id: u32) -> ULResult<()> {
        debug_assert!(self.texture_map.contains_key(&id));

        self.texture_map.remove(&id);

        Ok(())
    }

    fn create_render_buffer(&mut self, id: u32, render_buffer: RenderBuffer) -> ULResult<()> {
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

    fn destroy_render_buffer(&mut self, id: u32) -> ULResult<()> {
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

    fn destroy_geometry(&mut self, id: u32) -> ULResult<()> {
        debug_assert!(self.geometry_map.contains_key(&id));

        self.geometry_map.remove(&id);

        Ok(())
    }

    fn update_command_list(&mut self, command_list: Vec<GPUCommand>) -> ULResult<()> {
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

    fn clear_render_buffer(&mut self, render_buffer_id: u32) -> ULResult<()> {
        todo!();
    }

    fn draw_geometry(
        &mut self,
        gpu_state: Box<GPUState>,
        geometry_id: u32,
        indices_offset: u32,
        indices_count: u32,
    ) -> ULResult<()> {
        debug_assert!(self.geometry_map.contains_key(&geometry_id));
        let (vertex_buffer, index_buffer) = self
            .geometry_map
            .get(&geometry_id)
            .ok_or(ULError::GPUFailedToGetGeometry)?;

        debug_assert!(self
            .render_buffer_map
            .contains_key(&gpu_state.render_buffer_id));
        let render_buffer = self
            .render_buffer_map
            .get(&gpu_state.render_buffer_id)
            .ok_or(ULError::GPUFailedToGetRenderBuffer)?;

        let index_buffer_slize = index_buffer
            .slice(indices_offset as u64..(indices_offset as u64 + indices_count as u64));

        let (render_texture, _) = self
            .texture_map
            .get(&render_buffer.texture_id)
            .ok_or(ULError::GPUFailedToGetRenderBuffer)?;

        let used_shader = match gpu_state.shader_type {
            ShaderType::Fill => &self.fill_shader_module,
            ShaderType::FillPath => &self.path_shader_module,
        };

        let scalar_data = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Scalar Uniform Buffer"),
            contents: bytemuck::cast_slice(&[gpu_state.uniform_scalar]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let vector_data = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vector Uniform Buffer"),
            contents: bytemuck::cast_slice(&[gpu_state.uniform_vector]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let clip_data = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Clip Uniform Buffer"),
            contents: bytemuck::cast_slice(&[gpu_state.clip]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

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

        for (i, row) in transformation.iter_mut().enumerate() {
            for (j, column) in row.iter_mut().enumerate() {
                for (k, row_orth) in orth_projection_matrix.iter().enumerate() {
                    *column += gpu_state.transform[i * 4 + k] * (*row_orth)[j];
                }
            }
        }

        let texture1 = if let Some(id) = gpu_state.texture_1_id {
            let (t, _) = self
                .texture_map
                .get(&id)
                .ok_or(ULError::GPUFailedToGetRenderBuffer)?;
            t
        } else {
            &self.empty_texture
        };
        let texture2 = if let Some(id) = gpu_state.texture_2_id {
            let (t, _) = self
                .texture_map
                .get(&id)
                .ok_or(ULError::GPUFailedToGetRenderBuffer)?;
            t
        } else {
            &self.empty_texture
        };
        let texture3 = if let Some(id) = gpu_state.texture_3_id {
            let (t, _) = self
                .texture_map
                .get(&id)
                .ok_or(ULError::GPUFailedToGetRenderBuffer)?;
            t
        } else {
            &self.empty_texture
        };

        let view = render_texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(wgpu::Color::BLACK),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        todo!();
    }

    fn update_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: DriverVertexBuffer,
        index_buffer: IndexBuffer,
    ) -> ULResult<()> {
        todo!();
    }

    fn create_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: DriverVertexBuffer,
        index_buffer: IndexBuffer,
    ) -> ULResult<()> {
        todo!();
    }

    pub(crate) fn render_bitmap(&mut self, texture_id: u32) -> ULResult<Cow<'_, [u8]>> {
        todo!();
    }
}
