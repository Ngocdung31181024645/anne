use book::{Metadata, ReadError};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::{BufReader, Error as IOError, Read};
use std::path::Path;
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

fn epub_read_metadata(path: &Path) -> Result<Metadata, ReadError> {
	let mut file = File::open(path)?;
	let mut zip = ZipArchive::new(file)?;
	let mut md_f = zip.by_name("content.opf")?;
	let mut buf = Vec::new();
	md_f.read_to_end(&mut buf)?;

	let buf_rdr = BufReader::new(&buf[..]);
	let mut rdr = Reader::from_reader(buf_rdr);

	let mut ev_buf = Vec::new();
	// @SPAGHETTI: This could become messy quickly
	let mut is_title = false;
	let mut found_title = false;
	let mut title = None;
	loop {
		match rdr.read_event(&mut ev_buf)? {
			Event::Start(ref e) => {
				match e.name() {
					b"dc:title" => {
						is_title = true;
						found_title = true;
					}
					_ => (),
				}
			}
			Event::End(ref e) => {
				match e.name() {
					b"dc:title" => is_title = false,
					_ => (),
				}
			}
			Event::Text(ref t) => {
				if is_title {
					title = Some(String::from_utf8_lossy(t).into_owned());
				};
			}
			Event::Eof => break,
			_ => (),
		}
	}

	if let Some(title) = title {
		Ok(Metadata { title })
	} else {
		Err(ReadError::MissingMetadata)
	}
}
