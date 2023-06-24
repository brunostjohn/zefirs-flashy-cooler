use std::{
    cmp::min,
    fs::{self, File},
    io::Write,
    thread::{self},
    time::Duration,
};

use crate::{CONFIG, RENDERER, SENSORS, SERVER, THEMES_PATH};
use color_thief::ColorFormat;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::Window;
use urlencoding::encode;

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
            version: "1.0.0".to_owned(),
            tested_on: None,
            customisable_parameters: vec![],
        };
    }
    get_theme(fs_name).unwrap()
}

#[tauri::command]
pub fn apply_default() {
    let sensors = SENSORS.lock().unwrap();
    let _ = sensors.pause();
    let mut server = SERVER.lock().unwrap();
    server.serve_path(None);
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct DownloadItem {
    pub ghPath: String,
    pub dlLink: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct ThemeManifest {
    pub fs_name: String,
    pub dlList: Vec<DownloadItem>,
    pub dlNum: u128,
    pub fileSizeKB: String,
    pub manifestDl: String,
    pub colour: String,
    pub image_src: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ProgressEvent {
    pub total_size: u64,
    pub tx_so_far: u64,
    pub file_name: String,
    pub file_count: u64,
    pub current_file: u64,
}

#[tauri::command]
pub async fn install_theme(fs_name: String, window: Window) -> Result<(), &'static str> {
    let mut theme_path = THEMES_PATH.clone().to_owned();
    theme_path.push(&fs_name);
    let manifest_file = match reqwest::get(
        "https://zfcapi.brunostjohn.com/theme/".to_string() + &encode(&fs_name).into_owned(),
    )
    .await
    {
        Ok(resp) => match resp.text().await {
            Ok(file) => file,
            Err(_) => return Err("Failed to convert theme manifest to string!"),
        },
        Err(_) => return Err("Failed to fetch theme manifest!"),
    };

    let manifest: ThemeManifest =
        serde_json::from_str(&manifest_file).or(Err("Failed to deserialise manifest!"))?;

    let total_theme_size = manifest
        .fileSizeKB
        .parse::<u64>()
        .or(Err("Failed to parse filesize."))?;

    let dl_list_len = manifest.dlList.len();
    let mut file_count = 1;
    let mut dld = 0;

    for file in manifest.dlList {
        let res = reqwest::Client::new()
            .get(file.dlLink)
            .send()
            .await
            .or(Err("Failed to start download of files!"))?;

        let total_size = match res.content_length() {
            Some(len) => len,
            None => 0,
        };

        let mut paths: Vec<&str> = file.ghPath.split("/").into_iter().collect();
        paths.remove(0);
        paths.remove(0);

        let mut dirs_only: Vec<&str> = paths.clone();
        let mut full_dir = theme_path.clone();
        dirs_only.remove(dirs_only.len() - 1);
        full_dir.extend(dirs_only);
        fs::create_dir_all(full_dir).or(Err("Failed to create dirs."))?;

        let mut location = theme_path.clone();
        location.extend(&paths);

        let mut file_handle = File::create(location).or(Err("Failed to create file."))?;
        let mut stream = res.bytes_stream();

        let mut downloaded = 0;

        while let Some(item) = stream.next().await {
            let chunk = item.or(Err("Failed to download chunk"))?;
            file_handle
                .write_all(&chunk)
                .or(Err("Failed to write to file."))?;
            let new = min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            window
                .emit(
                    "download-progress",
                    ProgressEvent {
                        total_size: total_theme_size,
                        tx_so_far: dld + downloaded as u64,
                        file_name: file.ghPath.clone(),
                        file_count: dl_list_len as u64,
                        current_file: file_count as u64,
                    },
                )
                .or(Err("Failed to emit event."))?;
        }
        dld += downloaded;
        file_count += 1;
    }

    window
        .emit("theme-installed", true)
        .or(Err("failed to emit event"))?;

    let _ = reqwest::get(
        "https://zfcapi.brunostjohn.com/theme/counts/".to_string()
            + &encode(&fs_name).into_owned()
            + "/fetch",
    )
    .await;
    Ok(())
}

#[tauri::command]
pub async fn uninstall_theme(fs_name: String, window: Window) -> Result<(), &'static str> {
    let mut theme_path = THEMES_PATH.clone();
    let mut server = SERVER.lock().unwrap();
    theme_path.push(&fs_name);

    if theme_path.as_path().exists() {
        fs::remove_dir_all(theme_path).or(Err("Failed to remove theme."))?;
        if server.now_serving() == fs_name {
            let renderer = RENDERER.lock().unwrap();
            server.serve_path(None);
            renderer.serve();
        }

        window
            .emit("delete-successful", true)
            .or(Err("Failed to emit event"))?;
        return Ok(());
    }

    Err("Files don't exist.")
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
    let mut server = SERVER.lock().unwrap();
    let mut config = CONFIG.lock().unwrap();

    let mut theme_path = THEMES_PATH.clone();
    theme_path.push(&fs_name);

    config.theme_path = Some(fs_name);

    if theme_path.as_path().exists() {
        server.serve_path(Some(theme_path));
        drop(server);
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Parameter {
    pub r#type: String,
    pub display_as: String,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: Option<String>,
    pub default: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,
    pub fs_name: String,
    pub colour: Option<String>,
    pub tested_on: Option<Vec<String>>,
    pub customisable_parameters: Vec<Parameter>,
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
                            customisable_parameters: vec![],
                            version: "0.0.0".to_owned(),
                            tested_on: None,
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
            version: res["version"]
                .as_str()
                .or(Some("1.0.0"))
                .unwrap()
                .to_string(),
            tested_on: serde_json::from_value(res["tested_on"].clone())
                .or::<Option<Vec<String>>>(Ok(None))
                .unwrap(),
            customisable_parameters: serde_json::from_value(res["customisable_parameters"].clone())
                .or::<Vec<Parameter>>(Ok(vec![]))
                .unwrap(),
        },
        Err(_) => Theme {
            name: fs_name.clone(),
            fs_name: fs_name,
            colour: None,
            description: "Failed to load theme.json".to_string(),
            author: "Failed to load".to_string(),
            tested_on: None,
            version: "0.0.0".to_owned(),
            customisable_parameters: vec![],
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
