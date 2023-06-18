use std::fs;

use crate::THEMES_PATH;

#[derive(serde::Serialize)]
pub struct Theme {
    pub name: String,
    pub fs_name: String,
}

#[tauri::command]
pub async fn get_all_themes() -> Result<Vec<Theme>, &'static str> {
    let theme_path = THEMES_PATH.as_path();

    let themes_iter = match fs::read_dir(theme_path) {
        Ok(vals) => vals,
        Err(_) => return Err("Failed to read themes!"),
    };

    let mut themes = vec![];

    for dir in themes_iter {
        match dir {
            Ok(item) => {
                let dir_path = item.path();

                let mut index_path = dir_path.clone();
                index_path.push("index.html");

                if index_path.as_path().exists() {
                    themes.push(Theme {
                        name: item.file_name().to_str().unwrap().to_string(),
                        fs_name: item.file_name().to_str().unwrap().to_string(),
                    });
                }
            }
            Err(_) => {}
        }
    }

    Ok(themes)
}
