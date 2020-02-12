#[derive(Debug)]
pub enum Error {
	AppDirs(app_dirs::AppDirsError),
	Io(std::io::Error),
	Serde(serde_json::error::Error),
}

impl From<app_dirs::AppDirsError> for Error {
	fn from(error: app_dirs::AppDirsError) -> Self {
		Error::AppDirs(error)
	}
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		Error::Io(error)
	}
}

impl From<serde_json::error::Error> for Error {
	fn from(error: serde_json::error::Error) -> Self {
		Error::Serde(error)
	}
}
