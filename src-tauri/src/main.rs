// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use directories::ProjectDirs;
use errors::Errors;
use db_api::Database;

mod app_api;
mod db_api;
mod errors;

fn main() -> Result<(), Errors> {
	let dirs = ProjectDirs::from("com", "varithdal", "Mydb")
		.ok_or_else(|| Errors::DirectoryCreationError)?;

	let db = Database::new(dirs.data_dir())?;
	db.initialize_tables()?;

    app_api::run(&db);

	Ok(())
}
