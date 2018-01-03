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

#[derive(Debug, Clone, Default)]
pub struct Metadata {
	// @CONSIDER: Would these be better as Cows?
	// @CONSIDER: Would these be better as [u8]?
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

// @NOTE: This must match the fields in Metadata
// @CONSIDER: There's probably an easier way to do this
#[derive(Copy, Clone)]
pub enum MetadataField {
	Title,
	Language,
	Identifier,
	Creator,
	Contributor,
	Publisher,
	Subject,
	Description,
	Date,
	TypeTag,
	Format,
	Source,
	Relation,
	Coverage,
	Rights,
}

impl MetadataField {
	pub fn lookup<'a>(self, meta: &'a Metadata) -> Option<&'a str> {
		match self {
			MetadataField::Title => Some(meta.title.as_str()),
			MetadataField::Language => Some(meta.language.as_str()),
			MetadataField::Identifier => Some(meta.identifier.as_str()),
			MetadataField::Creator => meta.creator.as_ref().map(|s| s.as_str()),
			MetadataField::Contributor => meta.contributor.as_ref().map(|s| s.as_str()),
			MetadataField::Publisher => meta.publisher.as_ref().map(|s| s.as_str()),
			MetadataField::Subject => meta.subject.as_ref().map(|s| s.as_str()),
			MetadataField::Description => meta.description.as_ref().map(|s| s.as_str()),
			MetadataField::Date => meta.date.as_ref().map(|s| s.as_str()),
			MetadataField::TypeTag => meta.type_tag.as_ref().map(|s| s.as_str()),
			MetadataField::Format => meta.format.as_ref().map(|s| s.as_str()),
			MetadataField::Source => meta.source.as_ref().map(|s| s.as_str()),
			MetadataField::Relation => meta.relation.as_ref().map(|s| s.as_str()),
			MetadataField::Coverage => meta.coverage.as_ref().map(|s| s.as_str()),
			MetadataField::Rights => meta.rights.as_ref().map(|s| s.as_str()),
			_ => panic!(),
		}
	}
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
		// TODO: It is more efficient to read only the metadata fields that we'll
		// actually be needing for a given computation
		let ft = Filetype::from_path(path);
		let meta = ft.read_metadata(path)?;
		Ok(Book {
			path: path,
			ft,
			meta,
		})
	}
}
