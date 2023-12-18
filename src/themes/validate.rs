use crate::utils::themes::paths::get_all_themes_path;
use std::path::Path;
use tokio::fs;

async fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).await.is_ok()
}

pub async fn validate_theme<S: AsRef<str>>(fs_name: S) -> bool {
    let theme_path = get_all_themes_path().join(fs_name.as_ref());

    let (index, preview, theme) = tokio::join!(
        file_exists(theme_path.join("index.html")),
        file_exists(theme_path.join("preview.jpg")),
        file_exists(theme_path.join("theme.json"))
    );

    index && preview && theme
}
