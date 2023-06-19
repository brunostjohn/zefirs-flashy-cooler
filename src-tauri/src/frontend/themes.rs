use std::{fs, thread, time::Duration};

use crate::{RENDERER, SERVER, THEMES_PATH};
use color_thief::ColorFormat;

#[tauri::command]
pub fn apply_theme(fs_name: String) {
    let renderer = RENDERER.lock().unwrap();
    let server = SERVER.lock().unwrap();

    let mut theme_path = THEMES_PATH.clone();
    theme_path.push(fs_name);

    if theme_path.as_path().exists() {
        server.serve_path(Some(theme_path));
        thread::sleep(Duration::from_millis(10));
        renderer.serve();
    }
}

fn get_image_buffer(img: image::DynamicImage) -> (Vec<u8>, ColorFormat) {
    match img {
        image::DynamicImage::ImageRgb8(buffer) => (buffer.to_vec(), color_thief::ColorFormat::Rgb),
        image::DynamicImage::ImageRgba8(buffer) => {
            (buffer.to_vec(), color_thief::ColorFormat::Rgba)
        }
        _ => unreachable!(),
    }
}

#[tauri::command]
pub async fn open_theme_folder() {
    std::process::Command::new("explorer")
        .arg(THEMES_PATH.to_str().unwrap())
        .spawn()
        .unwrap();
}

#[tauri::command]
pub async fn get_theme_folder() -> &'static str {
    return THEMES_PATH.to_str().unwrap();
}

#[derive(serde::Serialize)]
pub struct Theme {
    pub name: String,
    pub fs_name: String,
    pub colour: String,
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
                    let mut image_path = dir_path.clone();
                    image_path.push("preview.jpg");

                    let image_colour = match image::open(image_path) {
                        Ok(file) => {
                            let (buffer, color_type) = get_image_buffer(file);
                            let colors =
                                color_thief::get_palette(&buffer, color_type, 10, 10).unwrap();

                            format!(
                                "#{:02X?}{:02X?}{:02X?}",
                                255 - colors[0].r,
                                255 - colors[0].g,
                                255 - colors[0].b
                            )
                        }
                        Err(_) => "#FFFFFF".to_string(),
                    };

                    themes.push(Theme {
                        name: item.file_name().to_str().unwrap().to_string(),
                        fs_name: item.file_name().to_str().unwrap().to_string(),
                        colour: image_colour,
                    });
                }
            }
            Err(_) => {}
        }
    }

    Ok(themes)
}
