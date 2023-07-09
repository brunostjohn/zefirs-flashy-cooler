use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

use self::gpu::GPUDriver;

use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    device::QueueFlags,
    format::Format,
    image::ImageDimensions,
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator, MemoryUsage,
        StandardMemoryAllocator,
    },
    pipeline::graphics::vertex_input::Vertex,
};
use vulkano::{
    device::{physical::PhysicalDeviceType, Device, DeviceCreateInfo},
    image::StorageImage,
};
use vulkano::{
    device::{DeviceExtensions, Queue},
    VulkanLibrary,
};

use ul_sys::*;

#[path = "./gpu.rs"]
mod gpu;
use gpu::*;
#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct Vertex2F_4UB_2F {
    #[name("in_Position")]
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],

    #[name("in_Color")]
    #[format(R8G8B8_SRGB)]
    pub color: [u8; 4],

    #[name("in_TexCoord")]
    #[format(R32G32_SFLOAT)]
    pub obj: [f32; 2],
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct Vertex2F_4UB_2F_2F_28F {
    #[name("in_Position")]
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],

    #[name("in_Color")]
    #[format(R8G8B8_SRGB)]
    pub color: [u8; 4],

    #[name("in_TexCoord")]
    #[format(R32G32_SFLOAT)]
    pub obj: [f32; 2],

    #[name("in_ObjCoord")]
    #[format(R32G32_SFLOAT)]
    pub tex: [f32; 2],

    #[name("in_Data0")]
    #[format(R32G32B32A32_SFLOAT)]
    pub data0: [f32; 4],

    #[name("in_Data1")]
    #[format(R32G32B32A32_SFLOAT)]
    pub data1: [f32; 4],

    #[name("in_Data2")]
    #[format(R32G32B32A32_SFLOAT)]
    pub data2: [f32; 4],

    #[name("in_Data3")]
    #[format(R32G32B32A32_SFLOAT)]
    pub data3: [f32; 4],

    #[name("in_Data4")]
    #[format(R32G32B32A32_SFLOAT)]
    pub data4: [f32; 4],

    #[name("in_Data5")]
    #[format(R32G32B32A32_SFLOAT)]
    pub data5: [f32; 4],

    #[name("in_Data6")]
    #[format(R32G32B32A32_SFLOAT)]
    pub data6: [f32; 4],
}

pub trait VulkanVertexBuffer<T> {
    fn into_vulkan_vertex_buffer(
        self,
        alloc: &GenericMemoryAllocator<Arc<FreeListAllocator>>,
    ) -> Result<Subbuffer<[T]>, &'static str>;
}

impl VulkanVertexBuffer<Vertex2F_4UB_2F> for Vec<ULVertex_2f_4ub_2f> {
    fn into_vulkan_vertex_buffer(
        self,
        alloc: &GenericMemoryAllocator<Arc<FreeListAllocator>>,
    ) -> Result<Subbuffer<[Vertex2F_4UB_2F]>, &'static str> {
        let vlk_structs = self.iter().map(|x| Vertex2F_4UB_2F {
            position: x.pos,
            color: x.color,
            obj: x.obj,
        });

        let vertex_buffer = Buffer::from_iter(
            alloc,
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            vlk_structs,
        )
        .or(Err("Failed to allocate buffer"))?;

        Ok(vertex_buffer)
    }
}

impl VulkanVertexBuffer<Vertex2F_4UB_2F_2F_28F> for Vec<ULVertex_2f_4ub_2f_2f_28f> {
    fn into_vulkan_vertex_buffer(
        self,
        alloc: &GenericMemoryAllocator<Arc<FreeListAllocator>>,
    ) -> Result<Subbuffer<[Vertex2F_4UB_2F_2F_28F]>, &'static str> {
        let vlk_structs = self.iter().map(|x| Vertex2F_4UB_2F_2F_28F {
            position: x.pos,
            color: x.color,
            obj: x.obj,
            tex: x.tex,
            data0: x.data0,
            data1: x.data1,
            data2: x.data2,
            data3: x.data3,
            data4: x.data4,
            data5: x.data5,
            data6: x.data6,
        });

        let vertex_buffer = Buffer::from_iter(
            alloc,
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            vlk_structs,
        )
        .or(Err("Failed to allocate buffer"))?;

        Ok(vertex_buffer)
    }
}

pub enum VulkanGPUCommand {
    CreateTexture(u32, OwnedBitmap),
    UpdateTexture(u32, OwnedBitmap),
    DestroyTexture(u32),
    CreateRenderBuffer(u32, RenderBuffer),
    DestroyRenderBuffer(u32),
    CreateGeometry(
        u32,
        Option<Vec<ULVertex_2f_4ub_2f>>,
        Option<Vec<ULVertex_2f_4ub_2f_2f_28f>>,
        IndexBuffer,
    ),
    UpdateGeometry(
        u32,
        Option<Vec<ULVertex_2f_4ub_2f>>,
        Option<Vec<ULVertex_2f_4ub_2f_2f_28f>>,
        IndexBuffer,
    ),
    DestroyGeometry(u32),
    UpdateCommandList(Vec<GPUCommand>),
}

pub struct VulkanGPUDriverSender {
    next_texture_id: u32,
    next_render_buffer_id: u32,
    next_geometry_id: u32,
    sender: Sender<VulkanGPUCommand>,
}

impl GPUDriver for VulkanGPUDriverSender {
    fn begin_synchronize(&mut self) {
        todo!();
    }

    fn create_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: gpu::VertexBuffer,
        index_buffer: gpu::IndexBuffer,
    ) {
        match vertex_buffer.format {
            VertexBufferFormat::Format_2f_4ub_2f => {
                let (head, body, tail) = unsafe {
                    vertex_buffer
                        .buffer
                        .as_slice()
                        .align_to::<ULVertex_2f_4ub_2f>()
                };

                assert!(head.is_empty());
                assert!(tail.is_empty());

                let vertices = body.to_vec();

                self.sender
                    .send(VulkanGPUCommand::CreateGeometry(
                        geometry_id,
                        Some(vertices),
                        None,
                        index_buffer,
                    ))
                    .unwrap();
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

                let vertices = body.to_vec();

                self.sender
                    .send(VulkanGPUCommand::CreateGeometry(
                        geometry_id,
                        None,
                        Some(vertices),
                        index_buffer,
                    ))
                    .unwrap();
            }
        };
    }

    fn create_render_buffer(&mut self, render_buffer_id: u32, render_buffer: gpu::RenderBuffer) {
        self.sender
            .send(VulkanGPUCommand::CreateRenderBuffer(
                render_buffer_id,
                render_buffer,
            ))
            .unwrap();
    }

    fn create_texture(&mut self, texture_id: u32, bitmap: gpu::bitmap::OwnedBitmap) {
        self.sender
            .send(VulkanGPUCommand::CreateTexture(texture_id, bitmap))
            .unwrap();
    }

    fn destroy_geometry(&mut self, geometry_id: u32) {
        self.sender
            .send(VulkanGPUCommand::DestroyGeometry(geometry_id))
            .unwrap();
    }

    fn destroy_render_buffer(&mut self, render_buffer_id: u32) {
        self.sender
            .send(VulkanGPUCommand::DestroyRenderBuffer(render_buffer_id))
            .unwrap();
    }

    fn destroy_texture(&mut self, texture_id: u32) {
        self.sender
            .send(VulkanGPUCommand::DestroyTexture(texture_id))
            .unwrap();
    }

    fn end_synchronize(&mut self) {
        todo!();
    }

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
            .send(VulkanGPUCommand::UpdateCommandList(command_list))
            .unwrap();
    }

    fn update_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: gpu::VertexBuffer,
        index_buffer: gpu::IndexBuffer,
    ) {
        match vertex_buffer.format {
            VertexBufferFormat::Format_2f_4ub_2f => {
                let (head, body, tail) = unsafe {
                    vertex_buffer
                        .buffer
                        .as_slice()
                        .align_to::<ULVertex_2f_4ub_2f>()
                };

                assert!(head.is_empty());
                assert!(tail.is_empty());

                let vertices = body.to_vec();

                self.sender
                    .send(VulkanGPUCommand::UpdateGeometry(
                        geometry_id,
                        Some(vertices),
                        None,
                        index_buffer,
                    ))
                    .unwrap();
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

                let vertices = body.to_vec();

                self.sender
                    .send(VulkanGPUCommand::UpdateGeometry(
                        geometry_id,
                        None,
                        Some(vertices),
                        index_buffer,
                    ))
                    .unwrap();
            }
        };
    }

    fn update_texture(&mut self, texture_id: u32, bitmap: OwnedBitmap) {
        self.sender
            .send(VulkanGPUCommand::UpdateTexture(texture_id, bitmap))
            .unwrap();
    }
}

pub struct VulkanGPUDriverReceiver {
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
    device: Arc<Device>,
    queues: Box<dyn ExactSizeIterator<Item = Arc<Queue>>>,
    receiver: Receiver<VulkanGPUCommand>,
    allocator: GenericMemoryAllocator<Arc<FreeListAllocator>>,
    main_queue: Arc<Queue>,
    main_queue_family_index: u32,
}

impl VulkanGPUDriverReceiver {
    pub fn new(receiver: Receiver<VulkanGPUCommand>) -> Result<Self, &'static str> {
        let library = VulkanLibrary::new().or(Err("Failed to load Vulkan library!"))?;

        let creation_info = InstanceCreateInfo {
            ..Default::default()
        };

        let instance = Instance::new(library.clone(), creation_info)
            .or(Err("Failed to create Vulkan instance!"))?;

        let device_extensions = DeviceExtensions {
            ..DeviceExtensions::empty()
        };

        let (physical_device, _queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .position(|q| q.queue_flags.intersects(QueueFlags::COMPUTE))
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                ..Default::default()
            },
        )
        .or(Err("Failed to create device!"))?;

        let allocator = StandardMemoryAllocator::new_default(device.clone());

        let main_queue = queues.next().unwrap();

        Ok(Self {
            library,
            instance,
            device,
            queues: Box::new(queues),
            receiver,
            allocator,
            main_queue_family_index: main_queue.queue_family_index(),
            main_queue,
        })
    }

    pub fn get_memory_allocator(&self) -> &GenericMemoryAllocator<Arc<FreeListAllocator>> {
        return &self.allocator;
    }

    pub fn render(&mut self) -> Result<(), &'static str> {
        while let Ok(cmd) = self.receiver.try_recv() {
            match cmd {
                VulkanGPUCommand::CreateTexture(id, bitmap) => {
                    self.create_texture(id, bitmap)?;
                }
                VulkanGPUCommand::UpdateTexture(id, bitmap) => {
                    self.update_texture(id, bitmap)?;
                }
                VulkanGPUCommand::DestroyTexture(id) => {
                    self.destroy_texture(id)?;
                }
                VulkanGPUCommand::CreateRenderBuffer(id, render_buffer) => {
                    self.create_render_buffer(id, render_buffer)?;
                }
                VulkanGPUCommand::DestroyRenderBuffer(id) => {
                    self.destroy_render_buffer(id)?;
                }
                VulkanGPUCommand::CreateGeometry(id, vert_small, vert_large, index) => {
                    if let Some(small) = vert_small {
                        self.create_geometry(id, small, index)?;
                    } else if let Some(large) = vert_large {
                        self.create_geometry(id, large, index)?;
                    }
                }
                VulkanGPUCommand::UpdateGeometry(id, vert_small, vert_large, index) => {
                    if let Some(small) = vert_small {
                        self.update_geometry(id, small, index)?;
                    } else if let Some(large) = vert_large {
                        self.update_geometry(id, large, index)?;
                    }
                }
                VulkanGPUCommand::DestroyGeometry(id) => {
                    self.destroy_geometry(id)?;
                }
                VulkanGPUCommand::UpdateCommandList(command_list) => {
                    self.update_command_list(command_list)?;
                }
            }
        }

        Ok(())
    }

    fn create_texture(&mut self, id: u32, bitmap: OwnedBitmap) -> Result<(), &'static str> {
        if bitmap.is_empty() {
            // create empty texture
        } else {
            let pixels = bitmap
                .pixels()
                .ok_or("Failed to get bitmap pixels")?
                .to_vec();

            let upload_buffer = Buffer::from_iter(
                &self.allocator,
                BufferCreateInfo {
                    usage: BufferUsage::TRANSFER_SRC,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                pixels,
            )
            .or(Err("Failed to create texture upload buffer!"))?;

            let image = match bitmap.format() {
                BitmapFormat::A8Unorm => StorageImage::new(
                    &self.allocator,
                    ImageDimensions::Dim2d {
                        width: bitmap.width(),
                        height: bitmap.height(),
                        array_layers: 1,
                    },
                    Format::R8_UNORM,
                    Some(self.main_queue_family_index),
                ),

                BitmapFormat::Bgra8UnormSrgb => StorageImage::new(
                    &self.allocator,
                    ImageDimensions::Dim2d {
                        width: bitmap.width(),
                        height: bitmap.height(),
                        array_layers: 1,
                    },
                    Format::B8G8R8A8_SRGB,
                    Some(self.main_queue_family_index),
                ),
            }
            .or(Err("Failed to create image"))?;
        }
        Ok(())
    }

    fn update_texture(&mut self, id: u32, bitmap: OwnedBitmap) -> Result<(), &'static str> {
        Ok(())
    }

    fn destroy_texture(&mut self, id: u32) -> Result<(), &'static str> {
        Ok(())
    }

    fn create_render_buffer(
        &mut self,
        id: u32,
        render_buffer: RenderBuffer,
    ) -> Result<(), &'static str> {
        Ok(())
    }

    fn destroy_render_buffer(&mut self, id: u32) -> Result<(), &'static str> {
        Ok(())
    }

    fn create_geometry<T>(
        &mut self,
        id: u32,
        vertex: impl VulkanVertexBuffer<T>,
        index: IndexBuffer,
    ) -> Result<(), &'static str> {
        Ok(())
    }

    fn update_geometry<T>(
        &mut self,
        id: u32,
        vertex: impl VulkanVertexBuffer<T>,
        index: IndexBuffer,
    ) -> Result<(), &'static str> {
        Ok(())
    }

    fn destroy_geometry(&mut self, id: u32) -> Result<(), &'static str> {
        Ok(())
    }

    fn update_command_list(&mut self, command_list: Vec<GPUCommand>) -> Result<(), &'static str> {
        Ok(())
    }
}
