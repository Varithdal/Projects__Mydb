use crate::errors::{Errors};
use std::path::Path;
use rusqlite::{Connection};

pub struct Database {
	connection: Connection
}

impl Database {
	pub fn new(path: &Path) -> Result<Database, Errors> {
		let connection = Connection::open(path)
			.map_err(|_| Errors::SqlConnectionFailure)?;

		Ok(Database { connection })
	}

	pub fn init(&self) -> Result<(), Errors> {
		self.connection
			.execute("", ())
			.map_err(|_| Errors::SqlExecutionError)?;

		Ok(())
	}
}
