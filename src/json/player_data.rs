use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    #[serde(rename = "available-campaign-levels")]
    pub(crate) available_campaign_levels: Option<AvailableCampaignLevels>,
    pub(crate) tutorials: Option<Tutorials>,
    #[serde(rename = "service-username")]
    pub(crate) service_username: Option<String>,
    #[serde(rename = "service-token")]
    pub(crate) service_token: Option<String>,
    #[serde(rename = "last-used-filters")]
    pub(crate) last_used_filters: Option<LastUsedFilters>,
    #[serde(rename = "column-ordering")]
    pub(crate) column_ordering: Option<ColumnOrdering>,
    #[serde(rename = "editor-lua-snippets")]
    pub(crate) editor_lua_snippets: Option<Vec<EditorLuaSnippet>>,
    #[serde(rename = "last-played-version")]
    pub(crate) last_played_version: LastPlayedVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AvailableCampaignLevels {
    pub(crate) npe: Option<Npe>,
    #[serde(rename = "tight-spot")]
    pub(crate) tight_spot: Option<Npe>,
    #[serde(rename = "transport-belt-madness")]
    pub(crate) transport_belt_madness: Option<Npe>,
    pub(crate) tutorial: Option<Npe>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Npe {
    #[serde(rename = "level-01")]
    pub(crate) level_01: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColumnOrdering {
    #[serde(rename = "public-index")]
    pub(crate) public_index: Option<String>,
    #[serde(rename = "public-ascending")]
    pub(crate) public_ascending: Option<bool>,
    #[serde(rename = "lan-index")]
    pub(crate) lan_index: Option<String>,
    #[serde(rename = "lan-ascending")]
    pub(crate) lan_ascending: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditorLuaSnippet {
    pub(crate) name: Option<String>,
    pub(crate) code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastPlayedVersion {
    pub(crate) game_version: String,
    pub(crate) build_version: Option<i64>,
    pub(crate) build_mode: Option<String>,
    pub(crate) platform: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastUsedFilters {
    pub(crate) players: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) mods: Option<String>,
    pub(crate) friends: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tutorials {
    #[serde(rename = "stack-transfers")]
    pub(crate) stack_transfers: Option<StackTransfers>,
    #[serde(rename = "logistic-system-basic")]
    pub(crate) logistic_system_basic: Option<ConstructionRobots>,
    #[serde(rename = "construction-robots")]
    pub(crate) construction_robots: Option<ConstructionRobots>,
    #[serde(rename = "trains-basics")]
    pub(crate) trains_basics: Option<ConstructionRobots>,
    #[serde(rename = "trains-ghost-rail-planner")]
    pub(crate) trains_ghost_rail_planner: Option<ConstructionRobots>,
    #[serde(rename = "trains-stations")]
    pub(crate) trains_stations: Option<ConstructionRobots>,
    #[serde(rename = "trains-basic-signals")]
    pub(crate) trains_basic_signals: Option<ConstructionRobots>,
    #[serde(rename = "trains-advanced-signals")]
    pub(crate) trains_advanced_signals: Option<ConstructionRobots>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstructionRobots {
    pub(crate) status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackTransfers {
    pub(crate) status: Option<String>,
    pub(crate) count: Option<i64>,
}
