use std::{fs, path::PathBuf, thread, time::Duration};

use crate::{RENDERER, SERVER, THEMES_PATH};
use async_recursion::async_recursion;
use color_thief::ColorFormat;
use reqwest::header::USER_AGENT;
use serde_json::Value;

#[tauri::command]
pub fn now_serving() -> Theme {
    let server = SERVER.lock().unwrap();
    let fs_name = server.now_serving();

    if fs_name == "__DEFAULT__" {
        return Theme {
            name: "Default Theme".to_string(),
            fs_name: "__DEFAULT__".to_string(),
            colour: Some("#FFFFFF".to_string()),
            description: "The default theme that comes compiled into the app.".to_string(),
            author: "Bruno St. John".to_string(),
        };
    }
    get_theme(fs_name).unwrap()
}

#[tauri::command]
pub fn apply_default() {
    let server = SERVER.lock().unwrap();
    server.serve_path(None);
}

#[derive(Debug)]
struct DownloadElement {
    pub url: String,
    pub path_to_write: PathBuf,
}

#[async_recursion]
async fn recurse_and_append(
    search_path: String,
    theme_path: PathBuf,
) -> Result<Vec<DownloadElement>, &'static str> {
    println!("Searching for {:?}", search_path);

    let client = reqwest::Client::new();

    let endpoint = format!(
        "https://api.github.com/repos/brunostjohn/zefirs-flashy-cooler-themes/contents/Themes/{search_path}"
    );

    let body = match client.get(endpoint).header(USER_AGENT, "ZFC").send().await {
        Ok(resp) => resp,
        Err(_) => return Err("Failed to get theme information."),
    };

    let text = match body.text().await {
        Ok(txt) => txt,
        Err(_) => return Err("Failed to parse API response."),
    };

    let response = match serde_json::from_str::<Vec<Value>>(&text) {
        Ok(value) => value,
        Err(_) => return Err("Failed to deserialise API response to initial list."),
    };

    let mut results = vec![];

    for file_or_dir in response {
        let type_of = match &file_or_dir["type"] {
            serde_json::Value::String(value) => value,
            _ => return Err("Failed to deserialise API response file/dir!"),
        };

        match type_of.as_str() {
            "file" => {
                let url = match &file_or_dir["download_url"] {
                    serde_json::Value::String(url) => url,
                    _ => return Err("Failed to deserialise API response/file/url!"),
                };

                let git_path = match &file_or_dir["path"] {
                    serde_json::Value::String(path) => path,
                    _ => return Err("Failed to deserialise API response/file/git_path!"),
                };

                let mut path_elements: Vec<&str> = git_path.split("/").into_iter().collect();
                path_elements.remove(0);
                path_elements.remove(0);
                println!("{:?}", path_elements);

                let mut base_path = theme_path.clone();
                base_path.extend(path_elements);

                results.push(DownloadElement {
                    url: url.to_owned(),
                    path_to_write: base_path,
                })
            }
            "dir" => {
                let git_path = match &file_or_dir["path"] {
                    serde_json::Value::String(path) => path,
                    _ => return Err("Failed to deserialise API response/dir/git_path!"),
                };

                let mut path_elements: Vec<&str> = git_path.split("/").into_iter().collect();
                path_elements.remove(0);
                path_elements.remove(1);

                let location_string = path_elements.join("/");

                let recursed_vector =
                    recurse_and_append(location_string, theme_path.clone()).await?;

                results.extend(recursed_vector);
            }
            _ => {}
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn install_theme(fs_name: String) -> Result<(), &'static str> {
    let mut theme_path = THEMES_PATH.clone();
    theme_path.push(&fs_name);
    let download_list: Vec<DownloadElement> = recurse_and_append(fs_name, theme_path).await?;
    println!("{:?}", download_list);
    Ok(())
}

#[tauri::command]
pub async fn uninstall_theme(fs_name: String) {
    let mut theme_path = THEMES_PATH.clone();
    theme_path.push(fs_name);

    if theme_path.as_path().exists() {
        fs::remove_dir_all(theme_path).unwrap();
    }
}

#[tauri::command]
pub async fn does_theme_exist(fs_name: String) -> bool {
    let mut theme_path = THEMES_PATH.clone();
    theme_path.push(fs_name);

    if theme_path.as_path().exists() {
        return true;
    }
    false
}

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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub name: String,
    pub fs_name: String,
    pub colour: Option<String>,
    pub description: String,
    pub author: String,
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

                    let mut json_path = dir_path.clone();
                    json_path.push("theme.json");

                    let loaded = match fs::read_to_string(json_path) {
                        Ok(st) => st,
                        Err(_) => "{}".to_string(),
                    };

                    let mut manifest = match serde_json::from_str(&loaded) {
                        Ok(res) => res,
                        Err(_) => Theme {
                            name: item.file_name().to_str().unwrap().to_string(),
                            fs_name: item.file_name().to_str().unwrap().to_string(),
                            colour: None,
                            description: "Failed to load theme.json".to_string(),
                            author: "Failed to load".to_string(),
                        },
                    };

                    match manifest.colour {
                        Some(_) => {}
                        None => {
                            let image_colour = match image::open(image_path) {
                                Ok(file) => {
                                    let (buffer, color_type) = get_image_buffer(file);
                                    let colors =
                                        color_thief::get_palette(&buffer, color_type, 10, 10)
                                            .unwrap();

                                    format!(
                                        "#{:02X?}{:02X?}{:02X?}",
                                        255 - colors[0].r,
                                        255 - colors[0].g,
                                        255 - colors[0].b
                                    )
                                }
                                Err(_) => "#FFFFFF".to_string(),
                            };
                            manifest.colour = Some(image_colour);
                        }
                    }

                    themes.push(manifest);
                }
            }
            Err(_) => {}
        }
    }

    Ok(themes)
}

#[tauri::command]
pub fn get_theme(fs_name: String) -> Result<Theme, &'static str> {
    let mut theme_path = THEMES_PATH.clone();
    theme_path.push(&fs_name);

    let dir_path = theme_path;

    let mut index_path = dir_path.clone();
    index_path.push("index.html");

    let mut json_path = dir_path.clone();
    json_path.push("theme.json");

    let loaded = match fs::read_to_string(json_path) {
        Ok(st) => st,
        Err(_) => "{}".to_string(),
    };

    let mut manifest: Theme = match serde_json::from_str::<Value>(&loaded) {
        Ok(res) => Theme {
            name: res["name"].as_str().unwrap().to_string(),
            fs_name,
            colour: None,
            description: res["description"].as_str().unwrap().to_string(),
            author: res["author"].as_str().unwrap().to_string(),
        },
        Err(_) => Theme {
            name: fs_name.clone(),
            fs_name: fs_name,
            colour: None,
            description: "Failed to load theme.json".to_string(),
            author: "Failed to load".to_string(),
        },
    };

    let mut image_path = dir_path.clone();
    image_path.push("preview.jpg");

    match manifest.colour {
        Some(_) => {}
        None => {
            let image_colour = match image::open(image_path) {
                Ok(file) => {
                    let (buffer, color_type) = get_image_buffer(file);
                    let colors = color_thief::get_palette(&buffer, color_type, 10, 10).unwrap();

                    format!(
                        "#{:02X?}{:02X?}{:02X?}",
                        255 - colors[0].r,
                        255 - colors[0].g,
                        255 - colors[0].b
                    )
                }
                Err(_) => "#FFFFFF".to_string(),
            };
            manifest.colour = Some(image_colour);
        }
    }
    return Ok(manifest);
}
