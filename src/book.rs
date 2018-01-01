use filetypes::Filetype;
use std::io::Error as IOError;
use std::path::Path;
use zip::result::ZipError;
use quick_xml::errors::Error as XmlError;

#[derive(Debug, Clone)]
pub struct Book<'a> {
	pub path: &'a Path,
	pub ft: Filetype,
	pub meta: Metadata,
}

#[derive(Debug, Clone, Default)]
pub struct Metadata {
	// @CONSIDER: Would these be better as Cows?
	// @UNIMPLEMENTED: Optional attributes on tags
	// see: www.hxa.name/articles/content/epub-guide_hxa7241_2007.html
	pub title: String,
	pub language: String,
	pub identifier: String,
	pub creator: Option<String>,
	pub contributor: Option<String>,
	pub publisher: Option<String>,
	pub subject: Option<String>,
	pub description: Option<String>,
	pub date: Option<String>,
	pub type_tag: Option<String>,
	pub format: Option<String>,
	pub source: Option<String>,
	pub relation: Option<String>,
	pub coverage: Option<String>,
	pub rights: Option<String>,
}

#[derive(Debug)]
pub enum ReadError {
	NoExt,
	UnimplementedFiletype,
	IOError(IOError),
	ZipError(ZipError),
	XmlError(XmlError),
}

impl From<IOError> for ReadError {
	fn from(e: IOError) -> Self {
		ReadError::IOError(e)
	}
}

impl From<ZipError> for ReadError {
	fn from(e: ZipError) -> Self {
		ReadError::ZipError(e)
	}
}

impl From<XmlError> for ReadError {
	fn from(e: XmlError) -> Self {
		ReadError::XmlError(e)
	}
}

impl<'a> Book<'a> {
	pub fn from_path(path: &'a Path) -> Result<Self, ReadError> {
		let ft = match Filetype::from_path(path) {
			Some(ft) => ft,
			None => return Err(ReadError::NoExt),
		};

		let meta = ft.read_metadata(path)?;
		Ok(Book {
			path: path,
			ft,
			meta,
		})
	}
}
