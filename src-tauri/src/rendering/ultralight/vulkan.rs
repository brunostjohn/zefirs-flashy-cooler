use std::{
    borrow::Cow,
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
};

use self::gpu::GPUDriver;

use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferError, BufferUsage, Subbuffer},
    device::QueueFlags,
    format,
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator, MemoryUsage,
        StandardMemoryAllocator,
    },
    pipeline::graphics::vertex_input::Vertex,
};
use vulkano::{
    device::{physical::PhysicalDeviceType, Device, DeviceCreateInfo},
    format::Format,
};
use vulkano::{
    device::{DeviceExtensions, Queue},
    VulkanLibrary,
};

use ul_sys::*;

#[path = "./gpu.rs"]
mod gpu;
use gpu::*;

// pub enum VulkanVertexBuffer {
//     Format2f4ub2f(Vec<ULVertex_2f_4ub_2f>),
//     Format2f4ub2f2f28f(Vec<ULVertex_2f_4ub_2f_2f_28f>),
// }

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
        Option<Vertex2F_4UB_2F>,
        Option<Vertex2F_4UB_2F_2F_28F>,
        IndexBuffer,
    ),
    UpdateGeometry(
        u32,
        Option<Vertex2F_4UB_2F>,
        Option<Vertex2F_4UB_2F_2F_28F>,
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

    fn update_command_list(&mut self, command_list: Vec<gpu::GPUCommand>) {}

    fn update_geometry(
        &mut self,
        geometry_id: u32,
        vertex_buffer: gpu::VertexBuffer,
        index_buffer: gpu::IndexBuffer,
    ) {
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

        let (device, queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                ..Default::default()
            },
        )
        .or(Err("Failed to create device!"))?;

        let allocator = StandardMemoryAllocator::new_default(device.clone());

        Ok(Self {
            library,
            instance,
            device,
            queues: Box::new(queues),
            receiver,
            allocator,
        })
    }

    pub fn get_memory_allocator(&self) -> &GenericMemoryAllocator<Arc<FreeListAllocator>> {
        return &self.allocator;
    }
}
