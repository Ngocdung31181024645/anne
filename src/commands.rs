use book::{Book, ReadError};
use clap::ArgMatches;
use filetypes::Filetype;
use formatting::{Error as FormatError, DEFAULT_FMT};
use std::fmt::Debug;
use std::fs;
use std::io::Error as IOError;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
	ReadError(ReadError),
	FormatError(FormatError),
	IOError(IOError),
	InvalidArgument,
}

impl From<ReadError> for Error {
	fn from(e: ReadError) -> Self {
		Error::ReadError(e)
	}
}

impl From<FormatError> for Error {
	fn from(e: FormatError) -> Self {
		Error::FormatError(e)
	}
}

impl From<IOError> for Error {
	fn from(e: IOError) -> Self {
		Error::IOError(e)
	}
}

pub fn add(args: &ArgMatches, lib: &Path) -> Result<(), Error> {
	let paths = match args.values_of("FILES") {
		None => return Err(Error::InvalidArgument),
		Some(f) => f,
	}.map(Path::new)
		.collect::<Vec<&Path>>();
	let mut dest = lib.to_owned();

	for p in paths {
		// @TODO: Optionally go through directories
		match Filetype::from_path(p) {
			Filetype::Unknown => if !args.is_present("allow_unknown") {
				warn!(
					"Unknown filetype: {}",
					p.file_name().unwrap().to_string_lossy()
				);
				continue;
			},
			_ => (),
		};

		dest.push(p.file_name().unwrap().to_string_lossy().as_ref());
		if args.is_present("copy") {
			fs::copy(p, &dest)?;
		} else {
			fs::rename(p, &dest)?;
		}
		dest.pop();
	}

	Ok(())
}

pub fn view(args: &ArgMatches, lib: &Path) -> Result<(), Error> {
	let files = fs::read_dir(lib).unwrap();
	for file in files {
		let f = file.unwrap();
		let md = f.metadata().unwrap();
		if md.is_dir() {
			continue;
		};

		let path = f.path();
		let book = Book::from_path(path.as_path())?;
		// TODO: This should be an option
		let fmted_str = DEFAULT_FMT.format_book(&book)?;

		println!("{}", fmted_str);
	}

	Ok(())
}
