/*/* all the old code just thrown in here

pub enum Entry<'a> {
	Profiles(&'a Profile),
	Documents(&'a Document),
	Videos(&'a Video),
	Audio(&'a Audio),
	Images(&'a Image),
	Lists(&'a List)
}

impl<'a> Database<'a> {
	pub fn new(root_path: &'a Path) -> Result<Database<'a>, Error> {
		let connection = Connection::open(root_path)
			.map_err(|_| Error::SqlConnectionFailure)?;

		Ok(Database {
			connection: connection,
			root_path: root_path
		})
	}

	pub fn initialize_tables(&self) -> Result<(), Error> {
		self.connection
			.execute("PRAGMA foreign_keys = ON;", ())
			.map_err(|_| Error::SqlInitializationError)?;

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
			.map_err(|_| Error::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Media (
						id INTEGER PRIMARY KEY,
						profile_id INTEGER NOT NULL,
						media_name TEXT NOT NULL,
						file_path TEXT NOT NULL UNIQUE,
						media_type TEXT NOT NULL,
						format TEXT NOT NULL,
						author_name TEXT NOT NULL,
						description TEXT NOT NULL,
						length INTEGER,
						width INTEGER,
						height INTEGER,
						
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
			.map_err(|_| Error::SqlInitializationError)?;

		self.connection
			.execute(
				"
					CREATE TABLE IF NOT EXISTS Lists (
						id INTEGER PRIMARY KEY,
						profile_id INTEGER NOT NULL,
						list_name TEXT NOT NULL UNIQUE,
						file_path TEXT NOT NULL,
						description TEXT NOT NULL,

						UNIQUE (profile_id, id),

						FOREIGN KEY (profile_id) REFERENCES Profiles(id)
					);
				",
				()
			)
			.map_err(|_| Error::SqlInitializationError)?;

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
			.map_err(|_| Error::SqlInitializationError)?;

		Ok(())
	}

	fn insert_profile_into_profiles(&self, profile: &Profile) -> Result<(PathBuf, i64), Error> {
		let path = self.get_entry_path_profile(&profile.profile_name);

		let id = self.connection
			.query_row(
				"
					INSERT INTO Profiles (
						profile_name,
						file_path
					)
					VALUES (
						?1,
						?2
					)
				",
				params![
					profile.profile_name,
					path.to_str()
				],
				|row| row.get(0)
			)
			.map_err(|_| Error::SqlInsertionError)?;

		Ok((path, id))
	}

	fn insert_document_into_documents(&self, document: &Document) -> Result<(PathBuf, i64), Error> {
		let path = self.get_entry_path_media(
			&document.profile_name,
			&document.author_name,
			&document.media_name
		);

		let id = self.connection
			.query_row(
				"
					INSERT INTO Media (
						profile_name,
						media_name,
						author_name,
						description,
						format,
						file_path,
						media_type
					)
					VALUES (
						?1,
						?2,
						?3,
						?4,
						?5,
						?6,
						'Document'
					)
					RETURNING id;
				",
				params![
					document.profile_name,
					document.media_name,
					document.author_name,
					document.description,
					document.format.to_string(),
					path.to_str(),
				],
				|row| row.get(0)
			)
			.map_err(|_| Error::SqlInsertionError)?;

		Ok((path, id))
	}

	fn insert_image_into_images(&self, image: &Image) -> Result<(PathBuf, i64), Error> {
		let path = self.get_entry_path_media(
			&image.profile_name,
			&image.author_name,
			&image.media_name
		);

		let id = self.connection
			.query_row(
				"
					INSERT INTO Media (
						profile_name,
						media_name,
						author_name,
						description,
						width,
						height,
						format,
						file_path,
						media_type
					)
					VALUES (
						?1,
						?2,
						?3,
						?4,
						?5,
						?6,
						'Image'
					)
					RETURNING id;
				",
				params![
					image.profile_name,
					image.media_name,
					image.author_name,
					image.description,
					image.width,
					image.height,
					image.format.to_string(),
					path.to_str()
				],
				|row| row.get(0)
			)
			.map_err(|_| Error::SqlInsertionError)?;

		Ok((path, id))
	}

	fn insert_audio_into_audio(&self, audio: &Audio) -> Result<(PathBuf, i64), Error> {
		let path = self.get_entry_path_media(
			&audio.profile_name,
			&audio.author_name,
			&audio.media_name
		);

		let id = self.connection
			.query_row(
				"
					INSERT INTO Media (
						profile_name,
						media_name,
						author_name,
						description,
						length,
						format,
						file_path,
						media_type
					)
					VALUES (
						?1,
						?2,
						?3,
						?4,
						?5,
						?6,
						?7,
						'Audio'
					)
					RETURNING id;
				",
				params![
					audio.profile_name,
					audio.media_name,
					audio.author_name,
					audio.description,
					audio.length,
					audio.format.to_string(),
					path.to_str()
				],
				|row| row.get(0)
			)
			.map_err(|_| Error::SqlInsertionError)?;

		Ok((path, id))
	}

	fn insert_video_into_videos(&self, video: &Video) -> Result<(PathBuf, i64), Error> {
		let path = self.get_entry_path_media(
			&video.profile_name,
			&video.author_name,
			&video.media_name
		);

		let id = self.connection
			.query_row(
				"
					INSERT INTO Media (
						profile_name,
						media_name,
						author_name,
						description,
						length,
						width,
						height,
						format,
						file_path,
						media_type
					)
					VALUES (
						?1,
						?2,
						?3,
						?4,
						?5,
						?6,
						?7,
						?8,
						?9,
						'Video'
					)
					RETURNING id;
				",
				params![
					video.profile_name,
					video.media_name,
					video.author_name,
					video.description,
					video.length,
					video.width,
					video.height,
					video.format.to_string(),
					path.to_str()
				],
				|row| row.get(0)
			)
			.map_err(|_| Error::SqlInsertionError)?;

		Ok((path, id))
	}

	fn insert_list_into_lists(&self, list: &List) -> Result<(PathBuf, i64), Error> {
		let path = self.get_entry_path_list(&list.profile_name, &list.list_name);

		let id = self.connection
			.query_row(
				"
					INSERT INTO List (
						profile_name,
						list_name,
						author_name
					)
					VALUES (
						?1,
						?2,
						?3
					)
					RETURNING id
				",
				params![
					list.profile_name,
					list.list_name,
					list.author_name
				],
				|row| row.get(0)
			)
			.map_err(|_| Error::SqlInsertionError)?;

		Ok((path, id))
	}

	pub fn insert_entry_into_list(&self, list: List) -> Result<(), Error> {
		for (list_id, media_id) in list.items {
			self.connection
				.execute(
					"
					",
					params![]
				)
				.map_err(|_| Error::SqlInsertionError)?;
		}

		Ok(())
	}

	pub fn insert_entry_into_table(&self, entry: &Entry) -> Result<(PathBuf, i64), Error> {
		Ok(match entry {
			Entry::Profiles(profile) => self.insert_profile_into_profiles(&profile)?,
			Entry::Documents(document) => self.insert_document_into_documents(&document)?,
			Entry::Images(images) => self.insert_image_into_images(&images)?,
			Entry::Audio(audio) => self.insert_audio_into_audio(&audio)?,
			Entry::Videos(video) => self.insert_video_into_videos(&video)?,
			Entry::Lists(list) => self.insert_list_into_lists(&list)?
		})
	}

	fn get_entry_path_list(&self, profile_name: &str, list_name: &str) -> PathBuf {
		let mut path = PathBuf::from(self.root_path);

		path.push(profile_name);
		path.push("lists");
		path.push(list_name);

		return path;
	}

	fn get_entry_path_profile(&self, profile_name: &str) -> PathBuf {
		let mut path = PathBuf::from(self.root_path);

		path.push("profiles");
		path.push(profile_name);

		return path;
	}

	fn get_entry_path_media(&self, profile_name: &str, author_name: &str, media_name: &str) -> PathBuf {
		let mut path = PathBuf::from(self.root_path);

		path.push(profile_name);
		path.push("media");
		path.push(author_name);
		path.push(media_name);

		return path;
	}

	fn create_new_entry(&self, profile: &str, author: &str, name: &str) -> Result<PathBuf, Error> {
		let path = self.get_entry_path_media(profile, author, name);

		fs::create_dir_all(&path)
			.map_err(|_| Error::DirectoryCreationError)?;

		Ok(path)
	}
	
	fn delete_entry(&self, path: PathBuf) -> Result<(), Error> {
		fs::remove_dir(path)
			.map_err(|_| Error::DirectoryRemovalError)?;

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
*/
 */