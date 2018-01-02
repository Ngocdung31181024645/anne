use book::{Metadata, MetadataField, ReadError};
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
	Unknown,
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

	let mut meta = Metadata::default();
	let mut xml_st = None;
	let mut xml_buf = Vec::new();
	loop {
		match xml_rdr.read_event(&mut xml_buf) {
			Ok(Event::Start(ref e)) => match e.name() {
				b"dc:title" => xml_st = Some(MetadataField::Title),
				b"dc:language" => xml_st = Some(MetadataField::Language),
				b"dc:identifier" => xml_st = Some(MetadataField::Identifier),
				b"dc:creator" => xml_st = Some(MetadataField::Creator),
				b"dc:contributor" => xml_st = Some(MetadataField::Contributor),
				b"dc:publisher" => xml_st = Some(MetadataField::Publisher),
				b"dc:subject" => xml_st = Some(MetadataField::Subject),
				b"dc:description" => xml_st = Some(MetadataField::Description),
				b"dc:date" => xml_st = Some(MetadataField::Date),
				b"dc:type_tag" => xml_st = Some(MetadataField::TypeTag),
				b"dc:format" => xml_st = Some(MetadataField::Format),
				b"dc:source" => xml_st = Some(MetadataField::Source),
				b"dc:relation" => xml_st = Some(MetadataField::Relation),
				b"dc:coverage" => xml_st = Some(MetadataField::Coverage),
				b"dc:rights" => xml_st = Some(MetadataField::Rights),
				_ => (),
			},
			Ok(Event::End(_)) => xml_st = None,
			Ok(Event::Text(ref t)) => match xml_st {
				None => (),
				Some(MetadataField::Title) => meta.title = String::from_utf8_lossy(t).into_owned(),
				Some(MetadataField::Language) => {
					meta.language = String::from_utf8_lossy(t).into_owned()
				}
				Some(MetadataField::Identifier) => {
					meta.identifier = String::from_utf8_lossy(t).into_owned()
				}
				Some(MetadataField::Creator) => {
					meta.creator = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Contributor) => {
					meta.contributor = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Publisher) => {
					meta.publisher = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Subject) => {
					meta.subject = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Description) => {
					meta.description = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Date) => {
					meta.date = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::TypeTag) => {
					meta.type_tag = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Format) => {
					meta.format = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Source) => {
					meta.source = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Relation) => {
					meta.relation = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Coverage) => {
					meta.coverage = Some(String::from_utf8_lossy(t).into_owned())
				}
				Some(MetadataField::Rights) => {
					meta.rights = Some(String::from_utf8_lossy(t).into_owned())
				}
			},
			Ok(Event::Eof) => break,
			Err(e) => return Err(e.into()),
			_ => (),
		}
	}

	Ok(meta)
}
