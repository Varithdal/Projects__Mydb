use crate::db::formats::Formats;

trait entry {
}

pub struct Profile {
	id: Option<i64>,
	profile_name: String
}

impl entry for Profile {
}

pub struct Document {
	id: Option<i64>,
	profile_name: Option<String>,
	media_name: String,
	author_name: Option<String>,
	description: Option<String>,
	format: Formats
}

impl entry for Document {
}

pub struct Image {
	id: Option<i64>,
	profile_name: Option<String>,
	media_name: String,
	author_name: Option<String>,
	description: Option<String>,
	width: Option<String>,
	height: Option<String>,
	format: Formats
}

impl entry for Image {
}

pub struct Audio {
	id: Option<i64>,
	profile_name: Option<String>,
	media_name: String,
	author_name: Option<String>,
	description: Option<String>,
	length: Option<i64>,
	format: Formats
}

impl entry for Audio {
}

pub struct Video {
	id: Option<i64>,
	profile_name: String,
	media_name: Option<String>,
	author_name: Option<String>,
	description: Option<String>,
	length: Option<i64>,
	width: Option<i64>,
	height: Option<i64>,
	format: Formats
}

impl entry for Video {
}

type ListItems = Option<Vec<(i64, i64)>>;

pub struct List {
	id: Option<i64>,
	profile_name: Option<String>,
	list_name: String,
	author_name: Option<String>
}

impl entry for List {
}
