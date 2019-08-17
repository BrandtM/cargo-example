use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub errors: Vec<ErrorElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorElement {
    pub detail: String,
}

impl Default for Error {
    fn default() -> Self {
        Error::from("Unexpected error while deserializing crate. This can happen if the crate is broken. In this case you can't run examples from this crate!")
    }
}

impl From<&str> for Error {
    fn from(from_str: &str) -> Self {
        let error = ErrorElement {
            detail: String::from(from_str),
        };

        Error {
            errors: vec![error],
        }
    }
}
