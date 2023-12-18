use crate::{
    lifecycle::exit,
    services::{rendering::message::RendererMessage, sensors::SensorMessage},
};
use discord_sdk::Discord;
use smol_static::ServerMessage;
use std::sync::Arc;
use tachyonix::Sender;
use tauri::{
    AppHandle, CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tokio::{runtime::Handle, sync::RwLock, task};

use super::main_window::recreate_main_window;

pub fn build_tray() -> SystemTray {
    let open_window = CustomMenuItem::new("open_window".to_string(), "Open Window");
    let quit_app = CustomMenuItem::new("quit_app".to_string(), "Quit App");

    let menu = SystemTrayMenu::new()
        .add_item(open_window)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit_app);

    SystemTray::new()
        .with_menu(menu)
        .with_tooltip("Zefir's Flashy Cooler")
        .with_id("main_tray")
}

pub fn tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            if app.get_window("main").is_none() {
                recreate_main_window(app);
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit_app" => {
                let sender_renderer: State<'_, Sender<RendererMessage>> = app.state();
                let sender_sensors: State<'_, Sender<SensorMessage>> = app.state();
                let sender_server: State<'_, Sender<ServerMessage>> = app.state();
                let discord: State<'_, Arc<RwLock<Option<Discord>>>> = app.state();

                task::block_in_place(move || {
                    Handle::current().block_on(async {
                        exit(
                            app.get_window("main"),
                            app,
                            sender_renderer,
                            sender_sensors,
                            sender_server,
                            discord,
                        )
                        .await;
                    })
                });
            }
            "open_window" => {
                recreate_main_window(app);
            }
            _ => {}
        },
        _ => {}
    }
}
