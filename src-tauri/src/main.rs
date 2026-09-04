// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::Result;
use db::Db;
use error::Error;

mod api;
mod db;
mod app;
mod error;

fn main() -> Result<()> {
	let root = app::get_app_root_path()
		.map_err(|_| Error::DirectoryCreationError)?;

	let db = Db::new(&root)?;
	db.initialize_tables()?;

	app::run();

	Ok(())
}
