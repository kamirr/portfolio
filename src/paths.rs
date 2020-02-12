use app_dirs::{AppDataType, AppInfo, app_root};
use std::path::PathBuf;

const APP_INFO: AppInfo = AppInfo {
	name: "portfolio",
	author: "Kamil Koczurek",
};

#[derive(Debug)]
pub struct Paths {
	pub config: PathBuf,
	pub data: PathBuf
}

impl Paths {
	pub fn get() -> crate::Result<Self> {
		Ok(Paths {
			config: config()?,
			data: data()?,
		})
	}
}

fn config() -> crate::Result<PathBuf> {
	Ok(app_root(AppDataType::UserConfig, &APP_INFO)?)
}

fn data() -> crate::Result<PathBuf> {
	Ok(app_root(AppDataType::UserData, &APP_INFO)?)
}
