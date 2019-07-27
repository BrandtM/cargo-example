use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "crate")]
    pub response_crate: Crate,
    pub versions: Vec<Version>,
    pub keywords: Vec<Keyword>,
    pub categories: Vec<Category>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: String,
    pub category: String,
    pub slug: String,
    pub description: String,
    pub created_at: String,
    pub crates_cnt: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keyword {
    pub id: String,
    pub keyword: String,
    pub created_at: String,
    pub crates_cnt: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Badge {
    pub badge_type: String,
    pub attributes: HashMap<String, Option<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crate {
    pub id: String,
    pub name: String,
    pub updated_at: String,
    pub versions: Option<Vec<i32>>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub badges: Option<Vec<Badge>>,
    pub created_at: String,
    pub downloads: i32,
    pub recent_downloads: Option<i64>,
    pub max_version: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub links: CrateLinks,
    pub exact_match: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrateLinks {
    pub version_downloads: String,
    pub versions: Option<String>,
    pub owners: Option<String>,
    pub owner_team: Option<String>,
    pub owner_user: Option<String>,
    pub reverse_dependencies: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub id: i32,

    #[serde(rename = "crate")]
    pub version_crate: String,
    pub num: String,
    pub dl_path: String,
    pub readme_path: String,
    pub updated_at: String,
    pub created_at: String,
    pub downloads: i32,
    pub features: serde_json::Value,
    pub yanked: bool,
    pub license: Option<String>,
    pub links: VersionLinks,
    pub crate_size: Option<i32>,
    pub published_by: Option<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionLinks {
    pub dependencies: String,
    pub version_downloads: String,
    pub authors: String,
}
