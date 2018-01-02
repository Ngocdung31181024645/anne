use clap::ArgMatches;
use std::path::Path;
use book::{Book, ReadError};
use std::fs;
use formatting::{DEFAULT_FMT, Error as FormatError};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Error {
	ReadError(ReadError),
	FormatError(FormatError),
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

pub fn add(args: &ArgMatches, path: &Path) -> Result<(), Error> {
	unimplemented!()
}

pub fn view(args: &ArgMatches, path: &Path) -> Result<(), Error> {
	let files = fs::read_dir(path).unwrap();
	for file in files {
		let f = file.unwrap();
		let md = f.metadata().unwrap();
		if md.is_dir() { continue; };

		let path = f.path();
		let book = Book::from_path(path.as_path())?;
		let fmted_str = DEFAULT_FMT.format_book(&book)?;

		println!("{}", fmted_str);
	}

	Ok(())
}
