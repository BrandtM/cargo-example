mod response;
mod error;

use response::*;
use error::*;
use clap::{Arg, App};
use git2::Repository;
use std::fs::DirBuilder;
use std::process::Command;

fn main() {
    let matches = App::new("cargo-example")
		.version("0.1")
		.about("An easy example runner for cargo")
		.arg(Arg::with_name("project")
			.required(true)
			.index(1))
		.get_matches();

	let project = matches.value_of("project").unwrap();

	let request_url = format!("https://crates.io/api/v1/crates/{}", project);
	let crates_response = reqwest::get(&request_url).unwrap().text().unwrap();
	let serialized_response: Option<Response> = serde_json::from_str(&crates_response).unwrap_or(None);
	let response: Response;

	if serialized_response.is_none() {
		let serialized_error: Error = serde_json::from_str(&crates_response).unwrap();
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

	let output = Command::new("cargo")
		.arg("run")
		.arg("--example")
		.current_dir(example_path)
		.output()
		.unwrap();

	println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
	println!("stderr: {}", String::from_utf8(output.stderr).unwrap());
}
