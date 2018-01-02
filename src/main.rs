#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate quick_xml;
extern crate zip;

mod library;
mod filetypes;
mod book;
mod commands;
mod formatting;

use std::env;
use std::process;
use commands::{add, view};

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
	lib_path.push("anne");

	if !lib_path.is_dir() {
		info!("Creating library directory at: {}", lib_path.display());
		if let Err(e) = std::fs::create_dir(&lib_path) {
			error!("Failed to create library directory: {}", e);
			process::exit(1);
		};
	}

	let app = clap_app!(anne =>
		(@setting SubcommandRequiredElseHelp)
		(version: "0.0.1")
		(author: "zovt <zovt@posteo.de>")
		(about: "Anne (the Librarian) - a simple, fast ebook collection manager")
		(@subcommand add =>
			(about: "Add books to your collection")
			(@arg allow_unknown: -u --allow-unknown "Allow unknown formats when adding books")
			(@arg copy: -c --copy "Copy books instead of move them")
		)
		(@subcommand view =>
			(about: "View information about the books in your collection")
		)
	);

	let matches = app.get_matches();

	if let Err(s) = match matches.subcommand() {
		("add", Some(m)) => add(m, &lib_path),
		("view", Some(m)) => view(m, &lib_path),
		_ => unreachable!(),
	} {
		println!("{:?}", s);
		process::exit(1);
	}
}
