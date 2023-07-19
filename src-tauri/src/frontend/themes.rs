use std::{
    cmp::min,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread::{self},
    time::Duration,
};

use crate::{rendering::ThemeConfigItem, ThemesPath};
use color_thief::ColorFormat;
use futures_util::StreamExt;
use macros::inject;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::Window;
use tauri::{api::dialog::blocking::FileDialogBuilder, State};
use urlencoding::encode;

#[inject(sensor_tree)]
#[tauri::command]
pub fn get_all_sensors() -> Vec<crate::sensors::Hardware> {
    sensor_tree.clone()
}

#[inject(server, renderer, themes_path)]
#[tauri::command]
pub fn select_file_and_save(name: String, current: String, window: Window) -> Result<(), ()> {
    let dialog = FileDialogBuilder::default().set_title("Select file");
    let picked = match dialog.pick_file() {
        Some(file) => file,
        None => return Ok(()),
    };

    let now_serving = server.now_serving();
    drop(server);

    let current = current.replace('/', "");

    let mut config_path = themes_path.clone();
    config_path.push(now_serving);
    let mut manifest_path = config_path.clone();
    let mut theme_path = config_path.clone();
    let mut old_path = config_path.clone();
    old_path.push(current);

    let _ = fs::remove_file(old_path);

    config_path.push("config.json");
    manifest_path.push("theme.json");

    let ext = match Path::new(&picked).extension().and_then(OsStr::to_str) {
        Some(ext) => ext.to_owned(),
        None => return Ok(()),
    };

    let filename = uuid::Uuid::new_v4().to_string() + "." + &ext;
    theme_path.push(&filename);

    if fs::copy(picked, theme_path).is_err() {
        return Ok(());
    }

    let value = format!("/{filename}");

    let theme_config_unparsed = fs::read_to_string(&config_path).unwrap_or("".to_owned());

    let theme_config_parsed: Vec<ThemeConfigItem> =
        serde_json::from_str(&theme_config_unparsed).unwrap_or(Vec::new());

    let mut config_without_param: Vec<ThemeConfigItem> = theme_config_parsed
        .iter()
        .filter(|x| x.name != name)
        .map(|x| x.to_owned())
        .collect();

    let manifest_unparsed = fs::read_to_string(manifest_path).unwrap_or("".to_owned());

    let manifest: Theme = match serde_json::from_str::<Value>(&manifest_unparsed) {
        Ok(res) => Theme {
            name: res["name"].as_str().unwrap().to_string(),
            fs_name: "".to_owned(),
            colour: None,
            description: res["description"].as_str().unwrap().to_string(),
            author: res["author"].as_str().unwrap().to_string(),
            version: res["version"].as_str().unwrap_or("1.0.0").to_string(),
            tested_on: serde_json::from_value(res["tested_on"].clone()).unwrap_or(None),
            customisable_parameters: serde_json::from_value(res["customisable_parameters"].clone())
                .unwrap_or(Vec::new()),
        },
        Err(_) => Theme {
            name: "".to_owned(),
            fs_name: "".to_owned(),
            colour: None,
            description: "Failed to load theme.json".to_string(),
            author: "Failed to load".to_string(),
            tested_on: None,
            version: "0.0.0".to_owned(),
            customisable_parameters: vec![],
        },
    };

    let config_item_pre = manifest
        .customisable_parameters
        .iter()
        .filter(|x| x.name == name)
        .collect::<Vec<_>>();

    let config_item = config_item_pre.first().unwrap();

    let item = ThemeConfigItem {
        r#type: config_item.r#type.clone(),
        value: value.clone(),
        name,
    };

    config_without_param.push(item);

    let updated_stringified = serde_json::to_string(&config_without_param).unwrap_or("".to_owned());

    let _ = fs::write(config_path, updated_stringified);

    renderer.reload_theme_config();
    drop(renderer);

    let _ = window.emit("changed-file", value);

    Ok(())
}

#[inject(themes_path, server)]
#[tauri::command]
pub fn get_current_theme_parameter(name: String) -> ThemeConfigItem {
    let now_serving = server.now_serving();
    drop(server);
    let mut config_path = themes_path.clone();
    config_path.push(now_serving);
    let mut manifest_path = config_path.clone();
    config_path.push("config.json");
    manifest_path.push("theme.json");

    if config_path.exists() {
        let theme_config_unparsed = fs::read_to_string(config_path).unwrap_or("".to_owned());

        let theme_config_parsed: Vec<ThemeConfigItem> =
            serde_json::from_str(&theme_config_unparsed).unwrap_or(Vec::new());

        let config_item = theme_config_parsed
            .iter()
            .filter(|x| x.name == name)
            .cloned()
            .collect::<Vec<ThemeConfigItem>>();

        if let Some(item) = config_item.first() {
            return item.clone();
        }
    }

    let manifest_unparsed = fs::read_to_string(manifest_path).unwrap_or("".to_owned());

    let manifest: Theme = match serde_json::from_str::<Value>(&manifest_unparsed) {
        Ok(res) => Theme {
            name: res["name"].as_str().unwrap().to_string(),
            fs_name: "".to_owned(),
            colour: None,
            description: res["description"].as_str().unwrap().to_string(),
            author: res["author"].as_str().unwrap().to_string(),
            version: res["version"].as_str().unwrap_or("1.0.0").to_string(),
            tested_on: serde_json::from_value(res["tested_on"].clone()).unwrap_or(None),
            customisable_parameters: serde_json::from_value(res["customisable_parameters"].clone())
                .unwrap_or(Vec::new()),
        },
        Err(_) => Theme {
            name: "".to_owned(),
            fs_name: "".to_owned(),
            colour: None,
            description: "Failed to load theme.json".to_string(),
            author: "Failed to load".to_string(),
            tested_on: None,
            version: "0.0.0".to_owned(),
            customisable_parameters: vec![],
        },
    };

    let config_item_pre = manifest
        .customisable_parameters
        .iter()
        .filter(|x| x.name == name)
        .collect::<Vec<_>>();

    let config_item = config_item_pre.first().unwrap();

    ThemeConfigItem {
        r#type: config_item.r#type.clone(),
        value: config_item.default.clone().unwrap_or("".to_owned()),
        name: config_item.name.clone(),
    }
}

#[inject(server, themes_path, renderer)]
#[tauri::command]
pub fn apply_theme_parameter(name: String, value: String) {
    let now_serving = server.now_serving();
    drop(server);

    let mut config_path = themes_path.clone();
    config_path.push(now_serving);
    let mut manifest_path = config_path.clone();
    config_path.push("config.json");
    manifest_path.push("theme.json");

    let theme_config_unparsed = fs::read_to_string(&config_path).unwrap_or("".to_owned());

    let theme_config_parsed: Vec<ThemeConfigItem> =
        serde_json::from_str(&theme_config_unparsed).unwrap_or(Vec::new());

    let mut config_without_param: Vec<ThemeConfigItem> = theme_config_parsed
        .iter()
        .filter(|x| x.name != name)
        .map(|x| x.to_owned())
        .collect();

    let manifest_unparsed = fs::read_to_string(manifest_path).unwrap_or("".to_owned());

    let manifest: Theme = match serde_json::from_str::<Value>(&manifest_unparsed) {
        Ok(res) => Theme {
            name: res["name"].as_str().unwrap().to_string(),
            fs_name: "".to_owned(),
            colour: None,
            description: res["description"].as_str().unwrap().to_string(),
            author: res["author"].as_str().unwrap().to_string(),
            version: res["version"].as_str().unwrap_or("1.0.0").to_string(),
            tested_on: serde_json::from_value(res["tested_on"].clone()).unwrap_or(None),
            customisable_parameters: serde_json::from_value(res["customisable_parameters"].clone())
                .unwrap_or(Vec::new()),
        },
        Err(_) => Theme {
            name: "".to_owned(),
            fs_name: "".to_owned(),
            colour: None,
            description: "Failed to load theme.json".to_string(),
            author: "Failed to load".to_string(),
            tested_on: None,
            version: "0.0.0".to_owned(),
            customisable_parameters: vec![],
        },
    };

    let config_item_pre = manifest
        .customisable_parameters
        .iter()
        .filter(|x| x.name == name)
        .collect::<Vec<_>>();

    let config_item = config_item_pre.first().unwrap();

    let item = ThemeConfigItem {
        r#type: config_item.r#type.clone(),
        value,
        name,
    };

    config_without_param.push(item);

    let updated_stringified = serde_json::to_string(&config_without_param).unwrap_or("".to_owned());

    let _ = fs::write(config_path, updated_stringified);

    renderer.reload_theme_config();
    drop(renderer);
}

#[inject(server, themes_path)]
#[tauri::command]
pub fn now_serving() -> Theme {
    let fs_name = server.now_serving();
    let theme_path = themes_path.to_owned();

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
    get_theme_inner(theme_path, fs_name).unwrap()
}

#[inject(server)]
#[tauri::command]
pub fn apply_default() {
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
pub async fn install_theme(
    fs_name: String,
    window: Window,
    themes_path: State<'_, Arc<Mutex<ThemesPath>>>,
) -> Result<(), &'static str> {
    let mut themes_path = themes_path.clone().lock().unwrap().to_owned();
    themes_path.push(&fs_name);

    let manifest_file =
        match reqwest::get("https://zfcapi.brunostjohn.com/theme/".to_string() + &encode(&fs_name))
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

        let total_size = res.content_length().unwrap_or(0);

        let mut paths: Vec<&str> = file.ghPath.split('/').collect();
        paths.remove(0);
        paths.remove(0);

        let mut dirs_only: Vec<&str> = paths.clone();
        let mut full_dir = themes_path.clone();
        dirs_only.remove(dirs_only.len() - 1);
        full_dir.extend(dirs_only);
        fs::create_dir_all(full_dir).or(Err("Failed to create dirs."))?;

        let mut location = themes_path.clone();
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
                        tx_so_far: dld + downloaded,
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
        "https://zfcapi.brunostjohn.com/theme/counts/".to_string() + &encode(&fs_name) + "/fetch",
    )
    .await;
    Ok(())
}

#[inject(renderer, server, themes_path)]
#[tauri::command]
pub fn uninstall_theme(fs_name: String, window: Window) -> Result<(), &'static str> {
    let mut themes_path = themes_path.clone();
    themes_path.push(&fs_name);

    if themes_path.as_path().exists() {
        fs::remove_dir_all(themes_path).or(Err("Failed to remove theme."))?;
        if server.now_serving() == fs_name {
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

#[inject(themes_path)]
#[tauri::command]
pub fn does_theme_exist(fs_name: String) -> bool {
    let mut theme_path = themes_path.clone();
    theme_path.push(fs_name);

    if theme_path.as_path().exists() {
        return true;
    }
    false
}

#[inject(renderer, server, config, themes_path)]
#[tauri::command]
pub fn apply_theme(fs_name: String) {
    let mut theme_path = themes_path.clone();
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

#[inject(renderer)]
#[tauri::command]
pub fn select_port(port: usize) {
    renderer.send_port(port);
}

#[inject(themes_path)]
#[tauri::command]
pub fn open_theme_folder() -> Result<(), ()> {
    std::process::Command::new("explorer")
        .arg(themes_path.to_str().unwrap())
        .spawn()
        .unwrap();

    Ok(())
}

#[inject(themes_path)]
#[tauri::command]
pub fn get_theme_folder() -> Result<String, ()> {
    Ok(themes_path.to_str().unwrap().to_owned())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Parameter {
    pub r#type: String,
    pub display_as: String,
    pub name: String,
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

#[inject(themes_path)]
#[tauri::command]
pub fn get_all_themes() -> Result<Vec<Theme>, &'static str> {
    let theme_path = themes_path.as_path();

    let themes_iter = match fs::read_dir(theme_path) {
        Ok(vals) => vals,
        Err(_) => return Err("Failed to read themes!"),
    };

    let mut themes = vec![];

    for item in themes_iter.flatten() {
        let dir_path = item.path();

        let mut index_path = dir_path.clone();
        index_path.push("index.html");

        if index_path.as_path().exists() {
            let mut image_path = dir_path.clone();
            image_path.push("preview.jpg");

            let mut manifest = get_theme_inner(
                theme_path.to_path_buf(),
                item.file_name().to_string_lossy().to_string(),
            )?;

            match manifest.colour {
                Some(_) => {}
                None => {
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
                    manifest.colour = Some(image_colour);
                }
            }

            themes.push(manifest);
        }
    }

    Ok(themes)
}

#[inject(themes_path)]
#[tauri::command]
pub fn get_theme(fs_name: String) -> Result<Theme, &'static str> {
    let theme_path = themes_path.to_owned();

    get_theme_inner(theme_path, fs_name)
}

pub fn get_theme_inner(mut theme_path: PathBuf, fs_name: String) -> Result<Theme, &'static str> {
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
            version: res["version"].as_str().unwrap_or("1.0.0").to_string(),
            tested_on: serde_json::from_value(res["tested_on"].clone()).unwrap_or(None),
            customisable_parameters: serde_json::from_value(res["customisable_parameters"].clone())
                .unwrap_or(Vec::new()),
        },
        Err(_) => Theme {
            name: fs_name.clone(),
            fs_name,
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
    Ok(manifest)
}
