use filetypes::Filetype;
use quick_xml::errors::Error as XmlError;
use std::io::Error as IOError;
use std::path::Path;
use zip::result::ZipError;

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

#[derive(Debug)]
pub enum ReadError {
	NoExt,
	UnimplementedFiletype,
	MissingMetadata,
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
