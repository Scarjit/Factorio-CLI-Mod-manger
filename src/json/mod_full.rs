use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModFull {
    pub category: Option<String>,
    pub changelog: Option<String>,
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub downloads_count: Option<i64>,
    pub github_path: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<License>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub releases: Option<Vec<Release>>,
    pub score: Option<f64>,
    pub summary: Option<String>,
    pub tag: Option<Tag>,
    pub thumbnail: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    pub download_url: String,
    pub file_name: String,
    pub info_json: InfoJson,
    pub released_at: Option<String>,
    pub sha1: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoJson {
    pub dependencies: Vec<String>,
    pub factorio_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: Option<String>,
}
