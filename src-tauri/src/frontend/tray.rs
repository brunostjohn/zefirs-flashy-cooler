#[path = "../frontend/window.rs"]
mod window;
use window::recreate_main_window;

#[path = "../app_ops/lifecycle.rs"]
mod lifecycle;
use lifecycle::exit;

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

pub fn build_tray() -> SystemTrayMenu {
    let open = CustomMenuItem::new("open".to_string(), "Open Window");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit App");
    SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
}

pub fn tray_event_handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("we are so clicked");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                exit(&app.get_window("main").unwrap(), app);
            }
            "open" => {
                recreate_main_window(app);
            }
            _ => {}
        },
        _ => {}
    };
}
