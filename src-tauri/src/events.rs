use std::fmt;
use serde::Serialize;

#[derive(Debug)]
pub(crate) enum Event {
	PortStatus,
	DebugText,
	TerminalText,
}

impl fmt::Display for Event {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Clone, Serialize)]
pub(crate) struct PortStatusPayload {
	pub(crate) connected_port_paths: Vec<String>,
	pub(crate) disconnected_port_paths: Vec<String>,
}

#[derive(Clone, Serialize)]
pub(crate) struct TextPayload {
	pub(crate) text: String,
}

pub(crate) fn emit_event(window: &tauri::Window, event: Event, payload: impl Clone + Serialize) {
	window
		.emit(&*event.to_string(), payload)
		.expect("Should have emitted an event!");
}