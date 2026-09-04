use crate::error::Error;
use crate::app::Result;
use rusqlite::Connection;
use std::path::PathBuf;

mod entries;
mod formats;
mod internal;

pub struct Db<'a> {
	conn: Connection,
	root: &'a PathBuf
}

impl<'a> Db<'a> {
	pub fn new(app_root_path: &'a PathBuf) -> Result<Db<'a>> {
		let conn = Connection::open(app_root_path)
			.map_err(|_| Error::SqlConnectionFailure)?;

		Ok(Db {
			conn: conn,
			root: app_root_path
		})
	}

	pub fn initialize_tables(&self) -> Result<()> {
		Ok(())
	}
}

fn foo() {

}
