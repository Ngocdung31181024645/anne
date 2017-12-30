use filetypes::Filetype;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Book<'a> {
	pub path: &'a Path,
	pub ft: Filetype,
	pub meta: Metadata,
}

#[derive(Debug, Clone)]
pub struct Metadata {
	pub title: String,
}

impl<'a> Book<'a> {
	pub fn from_path(path: &'a Path) -> Option<Self> {
		Filetype::from_path(path)
			.and_then(|ft| ft.read_metadata(path).map(|meta| (ft, meta)))
			.map(|(ft, meta)| {
				Book {
					path: path,
					ft,
					meta,
				}
			})
	}
}
