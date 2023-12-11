use wgpu::{include_spirv, ShaderModuleDescriptor};

pub(crate) fn get_fill_shader() -> ShaderModuleDescriptor<'static> {
    include_spirv!(concat!(env!("OUT_DIR"), "/fill.spv"))
}

pub(crate) fn get_fill_path_shader() -> ShaderModuleDescriptor<'static> {
    include_spirv!(concat!(env!("OUT_DIR"), "/fill_path.spv"))
}

pub(crate) fn get_v2f_c4f_t2f_shader() -> ShaderModuleDescriptor<'static> {
    include_spirv!(concat!(env!("OUT_DIR"), "/v2f_c4f_t2f.spv"))
}

pub(crate) fn get_v2f_c4f_t2f_d28f_shader() -> ShaderModuleDescriptor<'static> {
    include_spirv!(concat!(env!("OUT_DIR"), "/v2f_c4f_t2f_d28f.spv"))
}
