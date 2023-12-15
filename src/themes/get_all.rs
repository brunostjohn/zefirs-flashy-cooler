use std::path::PathBuf;

use crate::utils::themes::{manifest::Theme, paths::get_default_theme_path};
use tokio::fs;

#[tauri::command]
pub async fn get_all_themes_handler() -> Result<Vec<Theme>, String> {
    get_all_themes().await.map_err(|e| e.to_string())
}

async fn get_all_themes() -> anyhow::Result<Vec<Theme>> {
    let mut theme_path: PathBuf = get_default_theme_path().into();
    theme_path.pop();
    theme_path.pop();
    theme_path.push("Themes");
    let mut themes_dir = fs::read_dir(theme_path).await?;
    let mut themes = vec![];

    while let Ok(Some(theme_dir)) = themes_dir.next_entry().await {
        if theme_dir.file_name() == "__DEFAULT__" {
            continue;
        }
        let theme_path = theme_dir.path();
        let theme_manifest_path = theme_path.join("theme.json");
        let theme_manifest = if let Ok(manifest) = fs::read_to_string(theme_manifest_path).await {
            manifest
        } else {
            continue;
        };
        let mut theme_manifest: Theme = if let Ok(manifest) = serde_json::from_str(&theme_manifest)
        {
            manifest
        } else {
            continue;
        };

        theme_manifest.fs_name = Some(
            theme_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );

        if theme_path.join("index.html").exists() && theme_path.join("preview.jpg").exists() {
            themes.push(theme_manifest);
        }
    }

    Ok(themes)
}
