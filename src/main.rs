mod response;
mod error;

use response::*;
use error::*;
use clap::{Arg, App, Values};
use git2::Repository;
use std::fs::DirBuilder;
use std::path::Path;
use std::process::{Command, Stdio};

/// Represent the kinds of cache errors that could occur.
/// Allows for granular control
#[derive(Debug)]
enum CacheError {
	NotFound,
	BrokenState
}

/// Attempt to load a crate from the local filesystem
/// Will fail if:
/// - the directory doesn't exist
/// - the directory exists but there's no git repo inside of it
fn try_load_from_cache(crate_name: &str) -> Result<Repository, CacheError> {
	let crate_path = format!("{}/.cargo-example/{}", env!("CARGO_HOME"), crate_name);
	
	if Path::new(&crate_path).exists() {
		return match Repository::open(crate_path) {
			Ok(repo) => Ok(repo),
			Err(_) => Err(CacheError::BrokenState)
		};
	} 
	
	Err(CacheError::NotFound)
}

fn main() {
    let matches = App::new("cargo-example")
		.version("0.1")
		.about("An easy example runner for cargo")
		.arg(Arg::with_name("project")
			.required(true)
			.index(1))
		.arg(Arg::with_name("additional_args")
			.multiple(true))
		.get_matches();

	let project = matches.value_of("project").unwrap();
	let example_args: Vec<&str> = matches.values_of("additional_args").unwrap_or(Values::default()).collect();

	let cache = try_load_from_cache(&project);

	match cache {
		Ok(repo) => println!("Good cache 👍"),
		Err(e) => println!("Something happened 🤔 : {:?}", e)
	}

	let request_url = format!("https://crates.io/api/v1/crates/{}", project);
	let crates_response = reqwest::get(&request_url).unwrap().text().unwrap();
	let serialized_response: Option<Response> = serde_json::from_str(&crates_response).unwrap_or(None);
	let response: Response;

	if serialized_response.is_none() {
		let serialized_error: Error = serde_json::from_str(&crates_response).unwrap_or(Error::default());
		dbg!(serialized_error);
		return;
	} else {
		response = serialized_response.unwrap();
	}

	let example_path = format!("{}/.cargo-example/{}", env!("CARGO_HOME"), response.response_crate.name);

	DirBuilder::new()
		.recursive(true)
		.create(&example_path)
		.unwrap();

	let example_repo: Option<Repository> = match Repository::open(&example_path) {
		Ok(repo) => Some(repo),
		Err(_) => None
	};

	if example_repo.is_none() {
		Repository::clone(&response.response_crate.repository, &example_path).unwrap();
	}

	Command::new("cargo")
		.arg("run")
		.arg("--example")
		.args(&example_args)
		.current_dir(example_path)
		.stdin(Stdio::piped())
		.spawn()
		.unwrap();
}
