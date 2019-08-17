use crate::error::*;
use crate::response::*;
use git2::Repository;
use std::fs::DirBuilder;

/// Retrieve a crate's git repository URL via the crates.io API
/// and clone it locally. Returns error details (see error.rs)
/// in case of an error
pub fn download(crate_name: &str) -> Result<String, Error> {
    let request_url = format!("https://crates.io/api/v1/crates/{}", crate_name);
    let crates_response = reqwest::get(&request_url).unwrap().text().unwrap();
    let serialized_response: Option<Response> =
        serde_json::from_str(&crates_response).unwrap_or(None);
    let response: Response;

    if serialized_response.is_none() {
        let serialized_error: Error =
            serde_json::from_str(&crates_response).unwrap_or(Error::default());
        return Err(serialized_error);
    } else {
        response = serialized_response.unwrap();
    }

    let repository: String;

    if response.response_crate.repository.is_none() {
        let error = Error::from("Crate does not have a repository. Aborting!");
        return Err(error);
    } else {
        repository = response.response_crate.repository.unwrap();
    }

    let example_path = format!(
        "{}/.cargo-example/{}",
        env!("CARGO_HOME"),
        response.response_crate.name
    );

    DirBuilder::new()
        .recursive(true)
        .create(&example_path)
        .unwrap();

    Repository::clone(&repository, &example_path).unwrap();
    Ok(example_path)
}
