extern crate env_logger;
#[macro_use]
extern crate log;

mod library;
mod filetypes;
mod book;


use book::Book;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
	env_logger::init().expect("Failed to initialize logging");

	let mut lib_path;
	if let Some(home_dir) = env::home_dir() {
		lib_path = home_dir;
	} else {
		error!("Could not get home directory");
		process::exit(1);
	};
	lib_path.push(".local");
	lib_path.push("share");
	lib_path.push("alexandria");

	if !lib_path.is_dir() {
		info!("Creating library directory at: {}", lib_path.display());
		if let Err(e) = std::fs::create_dir(&lib_path) {
			error!("Failed to create library directory: {}", e);
			process::exit(1);
		};
	}

	let files = fs::read_dir(&lib_path).unwrap();
	for file in files {
		info!(
			"Book: {:?}",
			Book::from_path(file.unwrap().path().as_path())
		);
	}
}
