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
