use crate::{
    lifecycle::exit,
    services::{rendering::message::RendererMessage, sensors::SensorMessage},
};
use discord_sdk::Discord;
use smol_static::ServerMessage;
use std::sync::Arc;
use tachyonix::Sender;
use tauri::{AppHandle, Manager, RunEvent, State};
use tokio::{runtime::Handle, sync::RwLock, task};

#[allow(unused_variables)]
pub fn handle_events(app: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::Exit => {
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
        RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        RunEvent::WindowEvent { label, event, .. } => {}
        RunEvent::Ready => {}
        RunEvent::Resumed => {}
        RunEvent::MainEventsCleared => {}
        _ => {}
    }
}
