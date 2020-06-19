use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FullList {
    pub pagination: Option<serde_json::Value>,
    pub results: Vec<Result>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub name: String,
    pub title: String,
    pub owner: String,
    pub summary: String,
    pub downloads_count: i64,
    pub latest_release: LatestRelease,
    pub category: Option<Category>,
    pub score: Option<f64>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestRelease {
    pub download_url: String,
    pub file_name: String,
    pub info_json: InfoJson,
    pub released_at: String,
    pub version: String,
    pub sha1: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoJson {
    pub factorio_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "armor")]
    Armor,
    #[serde(rename = "balancing")]
    Balancing,
    #[serde(rename = "big-mods")]
    BigMods,
    #[serde(rename = "blueprints")]
    Blueprints,
    #[serde(rename = "cheats")]
    Cheats,
    #[serde(rename = "circuit-network")]
    CircuitNetwork,
    #[serde(rename = "defense")]
    Defense,
    #[serde(rename = "enemies")]
    Enemies,
    #[serde(rename = "environment")]
    Environment,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "helper-mods")]
    HelperMods,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "logistic-network")]
    LogisticNetwork,
    #[serde(rename = "logistics")]
    Logistics,
    #[serde(rename = "manufacture")]
    Manufacture,
    #[serde(rename = "mining")]
    Mining,
    #[serde(rename = "mod-packs")]
    ModPacks,
    #[serde(rename = "non-game-changing")]
    NonGameChanging,
    #[serde(rename = "oil")]
    Oil,
    #[serde(rename = "power-production")]
    PowerProduction,
    #[serde(rename = "scenarios")]
    Scenarios,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "trains")]
    Trains,
    #[serde(rename = "transportation")]
    Transportation,
    #[serde(rename = "utility")]
    Utility,
    #[serde(rename = "weapons")]
    Weapons,
}
