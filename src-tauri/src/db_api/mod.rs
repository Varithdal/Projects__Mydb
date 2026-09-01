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
			.execute("PRAGMA foreign_keys = ON;", ())
			.map_err(|_| Errors::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Profiles (
						id INTEGER PRIMARY KEY,
						name TEXT NOT NULL
					);
				",
				()
			)
			.map_err(|_| Errors::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Media (
						id INTEGER PRIMARY KEY,
						profile_id INTEGER NOT NULL,
						name TEXT NOT NULL,
						filepath TEXT NOT NULL,
						description TEXT,

						UNIQUE (profile_id, id),

						FOREIGN KEY (profile_id) REFERENCES Profiles(id)
					);
				",
				()
			)
			.map_err(|_| Errors::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Videos (
						id INTEGER PRIMARY KEY,
						length INTEGER,
						width INTEGER,
						height INTEGER,

						FOREIGN KEY (id) REFERENCES Media(id)
					);
				",
				()
			)
			.map_err(|_| Errors::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Audio (
						id INTEGER PRIMARY KEY,
						length INTEGER NOT NULL,

						FOREIGN KEY (id) REFERENCES Media(id)
					);
				",
				()
			)
			.map_err(|_| Errors::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Images (
						id INTEGER PRIMARY KEY,
						width INTEGER,
						height INTEGER,

						FOREIGN KEY (id) REFERENCES Media(id)
					);
				",
				()
			)
			.map_err(|_| Errors::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Lists (
						id INTEGER PRIMARY KEY,
						profile_id INTEGER NOT NULL,
						name TEXT NOT NULL,
						filepath TEXT NOT NULL,
						description TEXT,

						UNIQUE (profile_id, id),

						FOREIGN KEY (profile_id) REFERENCES Profiles(id)
					);
				",
				()
			)
			.map_err(|_| Errors::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS List_Media (
						profile_id INTEGER NOT NULL,
						list_id INTEGER NOT NULL,
						media_id INTEGER NOT NULL,

						FOREIGN KEY (profile_id) REFERENCES Profiles(id),
						FOREIGN KEY (profile_id, list_id) REFERENCES Lists(profile_id, id),
						FOREIGN KEY (profile_id, media_ID) REFERENCES Media(profile_id, id),
  
						PRIMARY KEY (profile_id, list_id, media_id)
					);
				",
				()
			)
			.map_err(|_| Errors::SqlInitializationError)?;

		Ok(())
	}
}
