use std::path::PathBuf;

pub fn get_app_path() -> anyhow::Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.pop();

    Ok(path)
}
