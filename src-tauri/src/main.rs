mod projector;

use projector::{list_ports, OptomaProjector};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

struct ProjectorState(Mutex<Option<OptomaProjector>>);

#[derive(Serialize)]
struct SerialPortInfo {
    port_name: String,
}

#[tauri::command]
fn get_serial_ports() -> Vec<SerialPortInfo> {
    list_ports()
        .into_iter()
        .map(|port| SerialPortInfo {
            port_name: port.port_name,
        })
        .collect()
}

#[tauri::command]
fn connect_projector(port_name: String, state: State<ProjectorState>) -> Result<(), String> {
    match OptomaProjector::new(&port_name) {
        Ok(projector) => {
            *state.0.lock().unwrap() = Some(projector);
            Ok(())
        }
        Err(e) => Err(format!("Connection failed: {}", e)),
    }
}

#[tauri::command]
fn send_command(command: String, state: State<ProjectorState>) -> Result<String, String> {
    if let Some(projector) = &mut *state.0.lock().unwrap() {
        projector.send_command(&command).map_err(|e| e.to_string())
    } else {
        Err("Not connected to projector".into())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(ProjectorState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            get_serial_ports,
            connect_projector,
            send_command
        ])
        .run(tauri::generate_context!())
        .expect("Error running Tauri");
}
