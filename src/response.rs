use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
	#[serde(rename = "crate")]
	pub response_crate: Crate,

	#[serde(rename = "versions")]
	pub versions: Vec<Version>,

	#[serde(rename = "keywords")]
	pub keywords: Vec<Option<serde_json::Value>>,

	#[serde(rename = "categories")]
	pub categories: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crate {
	#[serde(rename = "id")]
	pub id: String,

	#[serde(rename = "name")]
	pub name: String,

	#[serde(rename = "updated_at")]
	pub updated_at: String,

	#[serde(rename = "versions")]
	pub versions: Vec<i64>,

	#[serde(rename = "keywords")]
	pub keywords: Vec<Option<serde_json::Value>>,

	#[serde(rename = "categories")]
	pub categories: Vec<Option<serde_json::Value>>,

	#[serde(rename = "badges")]
	pub badges: Vec<Option<serde_json::Value>>,

	#[serde(rename = "created_at")]
	pub created_at: String,

	#[serde(rename = "downloads")]
	pub downloads: i64,

	#[serde(rename = "recent_downloads")]
	pub recent_downloads: i64,

	#[serde(rename = "max_version")]
	pub max_version: String,

	#[serde(rename = "description")]
	pub description: String,

	#[serde(rename = "homepage")]
	pub homepage: Option<serde_json::Value>,

	#[serde(rename = "documentation")]
	pub documentation: Option<serde_json::Value>,

	#[serde(rename = "repository")]
	pub repository: String,

	#[serde(rename = "links")]
	pub links: CrateLinks,

	#[serde(rename = "exact_match")]
	pub exact_match: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CrateLinks {
	#[serde(rename = "version_downloads")]
	pub version_downloads: String,

	#[serde(rename = "versions")]
	pub versions: Option<serde_json::Value>,

	#[serde(rename = "owners")]
	pub owners: String,

	#[serde(rename = "owner_team")]
	pub owner_team: String,

	#[serde(rename = "owner_user")]
	pub owner_user: String,

	#[serde(rename = "reverse_dependencies")]
	pub reverse_dependencies: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
	#[serde(rename = "id")]
	pub id: i64,

	#[serde(rename = "crate")]
	pub version_crate: String,

	#[serde(rename = "num")]
	pub num: String,

	#[serde(rename = "dl_path")]
	pub dl_path: String,

	#[serde(rename = "readme_path")]
	pub readme_path: String,

	#[serde(rename = "updated_at")]
	pub updated_at: String,

	#[serde(rename = "created_at")]
	pub created_at: String,

	#[serde(rename = "downloads")]
	pub downloads: i64,

	#[serde(rename = "features")]
	pub features: Features,

	#[serde(rename = "yanked")]
	pub yanked: bool,

	#[serde(rename = "license")]
	pub license: String,

	#[serde(rename = "links")]
	pub links: VersionLinks,

	#[serde(rename = "crate_size")]
	pub crate_size: i64,

	#[serde(rename = "published_by")]
	pub published_by: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Features {}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionLinks {
	#[serde(rename = "dependencies")]
	pub dependencies: String,

	#[serde(rename = "version_downloads")]
	pub version_downloads: String,

	#[serde(rename = "authors")]
	pub authors: String,
}
