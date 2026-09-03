#[derive(Debug)]
pub enum Errors {
	SqlConnectionFailure,
	SqlInitializationError,
	SqlSelectionError,
	SqlInsertionError,
	DirectoryCreationError,
	DirectoryRemovalError
}
