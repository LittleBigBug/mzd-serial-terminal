use crate::{ error };
use crate::events::{
	emit_event,
	Event,
	TextPayload,
	PortStatusPayload,
};

use std::thread;
use std::io::Read;
use std::{ fmt, time };
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread::JoinHandle;

use regex::Regex;
use serde::{ Deserialize, Serialize };
use serialport::{ SerialPort, SerialPortType };

struct OSerialPortType(SerialPortType);

impl fmt::Display for OSerialPortType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct PortInfo {
	pub(crate) name: String,
	pub(crate) port_type: String,
	pub(crate) product_name: Option<String>,
	pub(crate) manufacturer: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct SuggestedPorts {
	pub(crate) all_ports: Vec<PortInfo>,
	pub(crate) suggested_ports: Vec<PortInfo>,
}

pub(crate) fn get_ports() -> error::Result<Vec<PortInfo>> {
	Ok(serialport::available_ports()?
		.iter()
		.map(|port| {
			let port_type = port.port_type.clone();

			PortInfo {
				name: port.port_name.clone(),
				product_name:
					match &port_type {
						SerialPortType::UsbPort(usb_info) => usb_info.product.clone(),
						_ => None,
					},
				manufacturer:
					match &port_type {
						SerialPortType::UsbPort(usb_info) => usb_info.manufacturer.clone(),
						_ => None,
					},
				port_type: OSerialPortType(port_type).to_string(),
			}
		})
		.collect())
}

pub(crate) fn start_port_watcher(window: tauri::Window) -> JoinHandle<()> {
	thread::spawn(move || {
		let mut cur_port_paths: Vec<String> = Vec::new();

		loop {
			let port_res = serialport::available_ports();

			if port_res.is_err() {
				thread::sleep(time::Duration::from_millis(1000));
				continue;
			}

			let new_port_paths: Vec<String> = port_res
				.unwrap()
				.iter()
				.map(|port| port.port_name.clone())
				.collect();

			// Newly connected ports
			// todo; maybe detecting this solely on the path in the way I'm doing this isn't the best(?)
			let connected_port_paths: Vec<String> = new_port_paths
				.iter()
				.filter(|path| !cur_port_paths.contains(&path))
				.cloned()
				.collect();
			let disconnected_port_paths: Vec<String> = cur_port_paths
				.iter()
				.filter(|path| !new_port_paths.contains(&path))
				.cloned()
				.collect();

			cur_port_paths = new_port_paths.to_vec();

			// Emit an event on any port changes
			if !connected_port_paths.is_empty() || !disconnected_port_paths.is_empty() {
				emit_event(&window, Event::PortStatus, PortStatusPayload {
					connected_port_paths,
					disconnected_port_paths,
				});
			}

			thread::sleep(time::Duration::from_millis(300));
		}
	})
}

pub(crate) fn start_reader(window: tauri::Window, mut port: Box<dyn SerialPort>, kill_recv: Receiver<()>) -> JoinHandle<()> {
	thread::spawn(move || {
		let re = Regex::new(r"(\d{2}:?){2,3}\.\d{3} [\w>.]+\[\d*] .+\n?").unwrap();

		let mut line = String::new();
		let mut buf = [0_u8; 1000];

		let mut errors = 0;

		loop {
			match kill_recv.try_recv() {
				Ok(_) | Err(TryRecvError::Disconnected) => break,
				Err(TryRecvError::Empty) => { },
			}

			match port.read(buf.as_mut_slice()) {
				Ok(count) => {
					errors = 0;

					let mut term_text = String::new();
					let mut debug_text = String::new();

					let text = String::from_utf8_lossy(&buf[0..count]);

					let split = text.split("\n");
					let count = split.clone().count();

					let parts = split
						.filter(|part| !part.is_empty());

					// Parse each line
					for (i, part) in parts.enumerate() {
						// If not the first or last element (not \n)
						if i > 0 && (i + 1) < count {
							// commit line to term_text
							term_text.push_str(&*(line.clone() + "\n"));
							line.clear();
						}

						let take =
							// (Attempt to) Filter out Mazda debug messages
							// todo; fuzzy matching?
							match re.find(part) {
								Some(m) => Some(m.start()),
								None => None,
							};

						match take {
							Some(take) => {
								if take > 0 {
									// Add (hopefully) non-debug messages to the constructing line
									line.push_str(&part[..take]);
								}

								// Add debug messages to debug text
								debug_text.push_str(&part[take..]);
							},
							None => {
								line.push_str(part);
							},
						}
					}

					if term_text.len() > 0 {
						emit_event(&window, Event::TerminalText, TextPayload { text: term_text.clone() });
						term_text.clear();
					}

					// If there are committed lines, send event(s)
					if term_text.len() > 0 {
						emit_event(&window, Event::TerminalText, TextPayload { text: term_text.clone() });
						term_text.clear();
					}
					if debug_text.len() > 0 {
						emit_event(&window, Event::DebugText, TextPayload { text: debug_text.clone() });
						debug_text.clear();
					}
					// emit_event(&window, Event::TerminalText, TextPayload { text: text.parse().unwrap() });
				},
				Err(e) => {
					if errors >= 5 { break; }
					// errors += 1;
					eprintln!("{:?}", e)
				},
			}
		}
	})
}