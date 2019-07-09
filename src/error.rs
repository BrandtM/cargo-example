use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
	#[serde(rename = "errors")]
	pub errors: Vec<ErrorElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorElement {
	#[serde(rename = "detail")]
	pub detail: String,
}

impl Default for Error {
	fn default() -> Self {
		let error = ErrorElement {
			detail: String::from("Unexpected error while deserializing crate. This can happen if the crate is broken. In this case you can't run examples from this crate!"),
		};

		Error {
			errors: vec![error],
		}
	}
}