#[derive(Debug)]
pub enum Errors {
	SqlConnectionFailure,
	SqlExecutionError,
	DirectoryCreationError
}
