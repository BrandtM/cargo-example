mod response;
mod error;

use response::*;
use error::*;
use clap::{Arg, App};
use git2::Repository;
use std::fs::DirBuilder;

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

	Repository::clone(&response.response_crate.repository, &example_path).unwrap();
}
