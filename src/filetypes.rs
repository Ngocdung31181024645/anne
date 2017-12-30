use book::Metadata;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub enum Filetype {
	EPub,
	DjVu,
	Comic(ComicCompression),
}

#[derive(Debug, Copy, Clone)]
pub enum ComicCompression {
	Rar,
	Zip,
	Tar,
	SevenZ,
}

impl Filetype {
	pub fn from_path(path: &Path) -> Option<Self> {
		path.extension()
			.and_then(|e| e.to_str())
			.map(|ext| ext.to_lowercase())
			.and_then(|ext| match ext.as_ref() {
				"epub" => Some(Filetype::EPub),
				"djvu" => Some(Filetype::DjVu),
				"cbz" => Some(Filetype::Comic(ComicCompression::Zip)),
				"cbr" => Some(Filetype::Comic(ComicCompression::Rar)),
				"cb7" => Some(Filetype::Comic(ComicCompression::SevenZ)),
				"cbt" => Some(Filetype::Comic(ComicCompression::Tar)),
				_ => None,
			})
	}

	pub fn read_metadata(self, path: &Path) -> Option<Metadata> {
		Some(Metadata { title: String::new() })
	}
}
