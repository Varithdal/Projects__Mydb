pub enum Formats {
	TXT, PDF, EPUB, DJVU,
	JPEG, PNG, GIF, TIFF, WEBP, SVG,
	WAV, FLAC, MP3, AAC,
	MP4, MKV
}

impl Formats {
	pub fn to_string(&self) -> String {
		String::from(
			match self {
				Formats::TXT => "TXT",
				Formats::PDF => "PDF",
				Formats::EPUB => "EPUB",
				Formats::DJVU => "DJVU",
				Formats::JPEG => "JPEG",
				Formats::PNG => "PNG",
				Formats::GIF => "GIF",
				Formats::TIFF => "TIFF",
				Formats::WEBP => "WEBP",
				Formats::SVG => "SVG",
				Formats::WAV => "WAV",
				Formats::FLAC => "FLAC",
				Formats::MP3 => "MP3",
				Formats::AAC => "AAC",
				Formats::MP4 => "MP4",
				Formats::MKV => "MKV"
			}
		)
	}
}
