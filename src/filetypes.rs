use book::{Metadata, ReadError};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;

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

	pub fn read_metadata(self, path: &Path) -> Result<Metadata, ReadError> {
		match self {
			Filetype::EPub => epub_read_metadata(path),
			_ => Err(ReadError::UnimplementedFiletype),
		}
	}
}

#[derive(Debug, Clone, Default)]
struct EpubMetadata {
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

impl Into<Metadata> for EpubMetadata {
	fn into(self) -> Metadata {
		let EpubMetadata {
			title,
			language,
			identifier,
			creator,
			contributor,
			publisher,
			subject,
			description,
			date,
			type_tag,
			format,
			source,
			relation,
			coverage,
			rights,
		} = self;

		Metadata {
			title,
			language,
			identifier,
			creator,
			contributor,
			publisher,
			subject,
			description,
			date,
			type_tag,
			format,
			source,
			relation,
			coverage,
			rights,
		}
	}
}

enum EpubXmlState {
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

fn epub_read_metadata(path: &Path) -> Result<Metadata, ReadError> {
	let file = File::open(path)?;
	let mut zip = ZipArchive::new(file)?;
	let mut zip_data = Vec::new();
	for i in 0..zip.len() {
		let mut file = zip.by_index(i)?;
		match PathBuf::from(file.name()).extension() {
			Some(s) => match &*s.to_string_lossy() {
				"opf" => {
					file.read_to_end(&mut zip_data)?;
					break;
				}
				_ => 0,
			},
			_ => 0,
		};
	}

	let buf_rdr = BufReader::new(&zip_data[..]);
	let mut xml_rdr = Reader::from_reader(buf_rdr);

	let mut epub_meta = EpubMetadata::default();
	let mut xml_st = None;
	let mut xml_buf = Vec::new();
	loop {
		match xml_rdr.read_event(&mut xml_buf) {
			Ok(Event::Start(ref e)) => match e.name() {
				b"dc:title" => xml_st = Some(EpubXmlState::Title),
				b"dc:language" => xml_st = Some(EpubXmlState::Language),
				b"dc:identifier" => xml_st = Some(EpubXmlState::Identifier),
				b"dc:creator" => xml_st = Some(EpubXmlState::Creator),
				b"dc:contributor" => xml_st = Some(EpubXmlState::Contributor),
				b"dc:publisher" => xml_st = Some(EpubXmlState::Publisher),
				b"dc:subject" => xml_st = Some(EpubXmlState::Subject),
				b"dc:description" => xml_st = Some(EpubXmlState::Description),
				b"dc:date" => xml_st = Some(EpubXmlState::Date),
				b"dc:type_tag" => xml_st = Some(EpubXmlState::TypeTag),
				b"dc:format" => xml_st = Some(EpubXmlState::Format),
				b"dc:source" => xml_st = Some(EpubXmlState::Source),
				b"dc:relation" => xml_st = Some(EpubXmlState::Relation),
				b"dc:coverage" => xml_st = Some(EpubXmlState::Coverage),
				b"dc:rights" => xml_st = Some(EpubXmlState::Rights),
				_ => (),
			},
			Ok(Event::End(_)) => xml_st = None,
			Ok(Event::Text(ref t)) => match xml_st {
				None => (),
				Some(EpubXmlState::Title) => {
					epub_meta.title = String::from_utf8_lossy(t).into_owned()
				}
				Some(EpubXmlState::Language) => {
					epub_meta.language = String::from_utf8_lossy(t).into_owned()
				}
				Some(EpubXmlState::Identifier) => {
					epub_meta.identifier = String::from_utf8_lossy(t).into_owned()
				}
				Some(EpubXmlState::Creator) => {
					epub_meta.creator = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Contributor) => {
					epub_meta.contributor = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Publisher) => {
					epub_meta.publisher = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Subject) => {
					epub_meta.subject = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Description) => {
					epub_meta.description = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Date) => {
					epub_meta.date = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::TypeTag) => {
					epub_meta.type_tag = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Format) => {
					epub_meta.format = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Source) => {
					epub_meta.source = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Relation) => {
					epub_meta.relation = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Coverage) => {
					epub_meta.coverage = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(EpubXmlState::Rights) => {
					epub_meta.rights = Some(String::from_utf8_lossy(t).into_owned())
				}
			},
			Ok(Event::Eof) => break,
			Err(e) => return Err(e.into()),
			_ => (),
		}
	}

	Ok(epub_meta.into())
}
