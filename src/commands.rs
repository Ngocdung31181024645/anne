use clap::ArgMatches;
use std::path::Path;
use book::Book;
use std::fs;

pub fn add(args: &ArgMatches, path: &Path) {
	unimplemented!()
}

pub fn view(args: &ArgMatches, path: &Path) {
	let files = fs::read_dir(path).unwrap();
	for file in files {
		let f = file.unwrap();
		let md = f.metadata().unwrap();
		if md.is_dir() { continue; };

		println!(
			"Book: {:?}",
			Book::from_path(f.path().as_path())
		);
	}
}
