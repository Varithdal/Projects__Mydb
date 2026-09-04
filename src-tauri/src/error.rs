#[derive(Debug)]
pub enum Error {
	SqlConnectionFailure,
	SqlInitializationError,
	SqlSelectionError,
	SqlInsertionError,
	DirectoryCreationError,
	DirectoryRemovalError
}
