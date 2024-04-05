use std::sync::{ Arc, Mutex };
use std::sync::mpsc::Sender;
use serialport::SerialPort;

pub(crate) struct TermState {
	pub(crate) port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
	pub(crate) reader_thread_killer: Arc<Mutex<Option<Sender<()>>>>,
}

impl TermState {
	pub(crate) fn new() -> TermState {
		TermState {
			port: Arc::new(Mutex::new(None)),
			reader_thread_killer: Arc::new(Mutex::new(None)),
		}
	}
}