use crate::error::Error;
use directories::ProjectDirs;
use std::result;
use std::path::PathBuf;

pub type Result<T> = result::Result<T, Error>;

pub fn get_app_root_path() -> Result<PathBuf> {
	let directories = ProjectDirs::from("com", "varithdal", "Mydb")
		.ok_or_else(|| Error::DirectoryCreationError)?;

	Ok(
		PathBuf::from(
			directories.data_dir()
		)
	)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
