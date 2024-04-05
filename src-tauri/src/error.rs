#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Tauri(#[from] tauri::Error),
	#[error(transparent)]
	Serial(#[from] tokio_serial::Error),
	#[error("Generic Error: {0}")]
	Generic(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::ser::Serializer {
		serializer.serialize_str(self.to_string().as_ref())
	}
}

pub(crate) type Result<T> = std::result::Result<T, Error>;