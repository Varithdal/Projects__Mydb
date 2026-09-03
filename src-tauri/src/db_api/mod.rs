use crate::errors::{Errors};
use std::path::{Path, PathBuf};
use rusqlite::{Connection, params};
use std::fs;

pub struct Database<'a> {
	connection: Connection,
	path: &'a Path
}

pub struct Profiles {
	profile_name: String
}

pub struct Documents {
	
}

pub struct Videos {

}

pub struct Audio {

}

pub struct Images {

}

pub struct Lists {

}

/*-- Documents
SELECT
   Profiles.profile_name,
   Media.media_name,
   Media.author_name,
   Media.description,
   Media.format
 FROM Profiles
 FULL OUTER JOIN Media
 ON Profiles.id = Media.id
 WHERE Profiles.profile_name = ?1
   AND Media.media_name = ?2
;


-- Image
SELECT
   Profiles.profile_name,
   Media.media_name,
   Media.author_name,
   Media.description,
   Media.width,
   Media.height,
   Media.format
 FROM Profiles
 FULL OUTER JOIN Media
 ON Profiles.id = Media.id
 WHERE Profiles.profile_name = ?1
   AND Media.media_name = ?2
;


-- Audio
SELECT
   Profiles.profile_name,
   Media.media_name,
   Media.author_name,
   Media.description,
   Media.length,
   Media.format
 FROM Profiles
 FULL OUTER JOIN Media
 ON Profiles.id = Media.id
 WHERE Profiles.profile_name = ?1
   AND Media.media_name = ?2
;


-- Video
SELECT
   Profiles.profile_name,
   Media.media_name,
   Media.author_name,
   Media.description,
   Media.length,
   Media.width,
   Media.height,
   Media.format
 FROM Profiles
 FULL OUTER JOIN Media
 ON Profiles.id = Media.id
 WHERE Profiles.profile_name = ?1
   AND Media.media_name = ?2
;
 */

pub enum Tables {
	Profiles(Profiles),
	Documents(Documents),
	Videos(Videos),
	Audio(Audio),
	Images(Images),
	Lists(Lists)
}

impl Tables {
	pub fn to_string(&self) -> String {
		String::from(
			match self {
				&Tables::Profiles(_) => "Profiles",
				&Tables::Documents(_) => "Documents",
				&Tables::Videos(_) => "Videos",
				&Tables::Audio(_) => "Audio",
				&Tables::Images(_) => "Images",
				&Tables::Lists(_) => "Lists"
			}
		)
	}
}

impl<'a> Database<'a> {
	pub fn new(path: &'a Path) -> Result<Database<'a>, Errors> {
		let connection = Connection::open(path)
			.map_err(|_| Errors::SqlConnectionFailure)?;

		Ok(Database {
			connection: connection,
			path: path
		})
	}

	// maybe combine or make a variant with new function
	pub fn initialize_tables(&self) -> Result<(), Errors> {
		self.connection
			.execute("PRAGMA foreign_keys = ON;", ())
			.map_err(|_| Errors::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Profiles (
						id INTEGER PRIMARY KEY,
						profile_name TEXT NOT NULL UNIQUE,
						file_path TEXT NOT NULL UNIQUE
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
						media_name TEXT NOT NULL,
						file_path TEXT NOT NULL UNIQUE, -- {persistent app data}/{profile}/{author}/{media}/{files}
						media_type TEXT NOT NULL, -- Document OR Image OR Audio OR Video
						format TEXT NOT NULL,
						author_name TEXT,
						description TEXT,
						length INTEGER, -- milliseconds -> duration_ms ?
						width INTEGER, -- pixels
						height INTEGER, -- pixels
						
						UNIQUE (profile_id, media_name),

						CHECK (
							( media_type = 'Document'
								AND format IN ('TXT', 'PDF', 'EPUB', 'DJVU')
								AND length IS NULL
								AND width IS NULL
								AND height IS NULL
							)
							
							OR ( media_type = 'Image'
								AND format IN ('JPEG', 'PNG', 'GIF', 'TIFF', 'WEBP', 'SVG')
								AND length IS NULL
								AND width IS NOT NULL
								AND height IS NOT NULL
							)

							OR ( media_type = 'Audio'
								AND format IN ('WAV', 'FLAC', 'MP3', 'AAC')
								AND length IS NOT NULL
								AND width IS NULL
								AND height IS NULL
							)
							
							OR ( media_type = 'Video'
								AND format IN ('MP4', 'MKV')
								AND length IS NOT NULL
								AND width IS NOT NULL
								AND height IS NOT NULL
							)
						)
						
						FOREIGN KEY (profile_id) REFERENCES Profiles(id)
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
						name TEXT NOT NULL UNIQUE,
						file_path TEXT NOT NULL,
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

	fn insert_into_profiles(&self, profile: &Profiles) -> Result<PathBuf, Errors> {
		let path = self.get_entry_path_profile(&profile.profile_name);

		self.connection
			.execute(
				"
					INSERT INTO Profiles (name)
					VALUES (?1)
				",
				params![profile.profile_name]
			)
			.map_err(|_| Errors::SqlInsertionError)?;

		Ok(path)
	}

	pub fn insert_into_table(&self, entry: &Tables) -> Result<PathBuf, Errors> {
		Ok(match entry {
			Tables::Profiles(x) => self.insert_into_profiles(&x)?,
			_ => todo!()
		})
	}

	fn get_entry_path_profile(&self, profile_name: &str) -> PathBuf {
		let mut path = PathBuf::from(self.path);

		path.push("profiles");
		path.push(profile_name);

		return path;
	}

	fn get_entry_path_media(&self, profile_name: &str, author_name: &str, media_name: &str) -> PathBuf {
		let mut path = PathBuf::from(self.path);

		path.push(profile_name);
		path.push(author_name);
		path.push(media_name);

		return path;
	}

	fn create_new_entry(&self, profile: &str, author: &str, name: &str) -> Result<PathBuf, Errors> {
		let path = self.get_entry_path_media(profile, author, name);

		fs::create_dir_all(&path)
			.map_err(|_| Errors::DirectoryCreationError)?;

		Ok(path)
	}
	
	fn delete_entry(&self, path: PathBuf) -> Result<(), Errors> {
		fs::remove_dir(path)
			.map_err(|_| Errors::DirectoryRemovalError)?;

		Ok(())
	}
}

/*-- Documents
SELECT
   Profiles.profile_name,
   Media.media_name,
   Media.author_name,
   Media.description,
   Media.format
 FROM Profiles
 FULL OUTER JOIN Media
 ON Profiles.id = Media.id
 WHERE Profiles.profile_name = ?1
   AND Media.media_name = ?2
;


-- Image
SELECT
   Profiles.profile_name,
   Media.media_name,
   Media.author_name,
   Media.description,
   Media.width,
   Media.height,
   Media.format
 FROM Profiles
 FULL OUTER JOIN Media
 ON Profiles.id = Media.id
 WHERE Profiles.profile_name = ?1
   AND Media.media_name = ?2
;


-- Audio
SELECT
   Profiles.profile_name,
   Media.media_name,
   Media.author_name,
   Media.description,
   Media.length,
   Media.format
 FROM Profiles
 FULL OUTER JOIN Media
 ON Profiles.id = Media.id
 WHERE Profiles.profile_name = ?1
   AND Media.media_name = ?2
;


-- Video
SELECT
   Profiles.profile_name,
   Media.media_name,
   Media.author_name,
   Media.description,
   Media.length,
   Media.width,
   Media.height,
   Media.format
 FROM Profiles
 FULL OUTER JOIN Media
 ON Profiles.id = Media.id
 WHERE Profiles.profile_name = ?1
   AND Media.media_name = ?2
;
 */
