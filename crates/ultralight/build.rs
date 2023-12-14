use spirv_compiler::*;

fn compile_shader_and_write_to_file(
    compiler: &mut Compiler,
    shader_path: &str,
    shader_kind: ShaderKind,
    output_path: &str,
) -> Result<(), CompilerError> {
    let _ = compiler.compile_from_file(shader_path, shader_kind, true)?;
    std::fs::copy(shader_path.to_owned() + ".spv", output_path)
        .expect("Failed to copy shader to output directory");
    std::fs::remove_file(shader_path.to_owned() + ".spv")
        .expect("Failed to remove shader from source directory");

    Ok(())
}

fn compile_shaders() -> Result<(), CompilerError> {
    let mut compiler = CompilerBuilder::new()
        .with_source_language(SourceLanguage::HLSL)
        .with_opt_level(OptimizationLevel::Performance)
        .build()
        .expect("Failed to initialise shaderc compiler");

    compile_shader_and_write_to_file(
        &mut compiler,
        "./shaders/path/fill.hlsl",
        ShaderKind::Fragment,
        &(std::env::var("OUT_DIR").unwrap() + "/fill.spv"),
    )?;

    compile_shader_and_write_to_file(
        &mut compiler,
        "./shaders/path/fill_path.hlsl",
        ShaderKind::Fragment,
        &(std::env::var("OUT_DIR").unwrap() + "/fill_path.spv"),
    )?;

    compile_shader_and_write_to_file(
        &mut compiler,
        "./shaders/vertex/v2f_c4f_t2f.hlsl",
        ShaderKind::Vertex,
        &(std::env::var("OUT_DIR").unwrap() + "/v2f_c4f_t2f.spv"),
    )?;

    compile_shader_and_write_to_file(
        &mut compiler,
        "./shaders/vertex/v2f_c4f_t2f_d28f.hlsl",
        ShaderKind::Vertex,
        &(std::env::var("OUT_DIR").unwrap() + "/v2f_c4f_t2f_d28f.spv"),
    )?;

    Ok(())
}

fn print_compiler_error_with_panic(error: CompilerError) {
    match error {
        CompilerError::Log(log_error) => {
            let CompilationError { file, description } = log_error;
            panic!(
                "Failed to compile shader\nFile: {:#?}\nError: {}",
                file, description
            );
        }
        CompilerError::LoadError(load_error) => {
            panic!("Failed to load shader\nError: {}", load_error);
        }
        CompilerError::WriteError(write_error) => {
            panic!("Failed to write shader\nError: {}", write_error);
        }
    }
}

pub fn main() {
    ultralight_build::build();

    compile_shaders()
        .map_err(print_compiler_error_with_panic)
        .expect("Failed to compile shaders!");
}
