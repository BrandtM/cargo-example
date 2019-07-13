use git2::Repository;
use std::path::Path;

/// Represent the kinds of cache errors that could occur.
/// Allows for granular control
#[derive(Debug)]
pub enum CacheError {
    NotFound,
    BrokenState,
}

/// Attempt to load a crate's git repository from the local filesystem
/// Will fail if:
/// - the directory doesn't exist
/// - the directory exists but there's no git repo inside of it
pub fn try_load_from_cache(crate_name: &str) -> Result<Repository, CacheError> {
    let crate_path = format!("{}/.cargo-example/{}", env!("CARGO_HOME"), crate_name);

    if Path::new(&crate_path).exists() {
        return match Repository::open(crate_path) {
            Ok(repo) => Ok(repo),
            Err(_) => Err(CacheError::BrokenState),
        };
    }

    Err(CacheError::NotFound)
}

/// Get the path to the git repo.
/// Returns None if it couldn't be determined
pub fn repository_to_path(repo: Repository) -> Option<String> {
    let path = repo.path();
    let parent = path.parent();

    if let Some(parent_path) = parent {
        let slice = parent_path.to_str();

        return match slice {
            Some(s) => Some(String::from(s)),
            None => None,
        };
    }

    None
}
