#[derive(Debug)]
pub enum Errors {
	SqlConnectionFailure,
	SqlInitializationError,
	SqlExecutionError,
	DirectoryCreationError
}
