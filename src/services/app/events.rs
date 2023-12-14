use crate::{
    lifecycle::exit,
    services::{rendering::message::RendererMessage, sensors::SensorMessage},
};
use smol_static::ServerMessage;
use tachyonix::Sender;
use tauri::{AppHandle, Manager, RunEvent, State};

#[allow(unused_variables)]
pub fn handle_events(app: &AppHandle, event: RunEvent) {
    match event {
        RunEvent::Exit => {
            let sender_renderer: State<'_, Sender<RendererMessage>> = app.state();
            let sender_sensors: State<'_, Sender<SensorMessage>> = app.state();
            let sender_server: State<'_, Sender<ServerMessage>> = app.state();

            exit(
                app.get_window("main"),
                app,
                sender_renderer,
                sender_sensors,
                sender_server,
            );
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
