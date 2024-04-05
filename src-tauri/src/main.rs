// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod events;
mod error;
mod state;
mod serial;

use crate::state::TermState;
use crate::error::Error;
use crate::serial::{ start_reader, get_ports, start_port_watcher, SuggestedPorts };

use std::io::Write;
use std::sync::mpsc;
use std::time::Duration;

use tauri::Manager;
use serialport::{DataBits, FlowControl, Parity, StopBits};

#[tauri::command]
pub(crate) fn connect_serial(path: String, state: tauri::State<'_, TermState>, window: tauri::Window) -> error::Result<()> {
	close_serial(state.clone());

	let port = serialport::new(path, 115200)
		.stop_bits(StopBits::One)
		.data_bits(DataBits::Eight)
		.parity(Parity::None)
		.flow_control(FlowControl::Software) // XON/XOFF
		.timeout(Duration::from_millis(1000))
		.open()?;

	let win_clone = window.clone();
	let port_clone = port.try_clone().expect("Could not clone port!");

	let mut port_state = state.port.lock().expect("Could not lock port mutex");
	let mut rtk_state = state.reader_thread_killer.lock().expect("Could not lock rtk mutex");

	let (tx, rx) = mpsc::channel::<()>();

	start_reader(win_clone, port_clone, rx);

	*port_state = Some(port);
	*rtk_state = Some(tx);

	Ok(())
}

#[tauri::command]
pub(crate) fn close_serial(state: tauri::State<'_, TermState>) {
	let mut port_state = state.port.lock().expect("Could not lock mutex");
	let mut rtk_state = state.reader_thread_killer.lock().expect("Could not lock rtk mutex");

	match &mut *rtk_state {
		Some(kill) => { kill.send(()).ok(); },
		None => { },
	};

	match &mut *port_state {
		Some(_) => *port_state = None,
		None => { },
	}
}

#[tauri::command]
pub(crate) async fn write(input: String, state: tauri::State<'_, TermState>) -> error::Result<()> {
	let mut state = state.port.lock().expect("Could not lock mutex");
	match &mut *state {
		Some(ref mut writer) => {
			writer.write(input.as_bytes())?;
			Ok(())
		},
		None => Err(Error::Generic("Writing failed (no connection?)".into())),
	}
}

#[tauri::command]
pub(crate) fn suggest_serial_ports() -> error::Result<SuggestedPorts> {
	let all_ports = get_ports()?;
	let suggested_ports =
		all_ports
			.to_vec()
			.into_iter()
			.filter(|port| {
				match &port.product_name {
					Some(name) => name.contains("UART") || name.contains("CP210x"),
					None => false,
				}
			})
			.collect();

	Ok(
		SuggestedPorts {
			all_ports,
			suggested_ports,
		}
	)
}

fn main() {
	tauri::Builder::default()
		.manage(TermState::new())
		.setup(|app| {
			let main_window_clone = app
				.get_window("main")
				.expect("Couldn't get the main window!")
				.clone();

			start_port_watcher(main_window_clone);
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			write,
			close_serial,
			connect_serial,
			suggest_serial_ports,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
