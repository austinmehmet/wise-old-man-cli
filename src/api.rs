use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub exp: i32,
    pub id: i32,
    pub username: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub build: String,
    pub country: Option<String>,
    pub flagged: bool,
    pub ehp: f64,
    pub ehb: f64,
    #[serde(rename = "ttm")]
    pub time_to_max: f64,
    #[serde(rename = "tt200m")]
    pub time_to_200m: f64,
    pub last_imported_at: Option<String>,
    pub last_changed_at: Option<String>,
    pub registered_at: Option<String>,
    pub updated_at: Option<String>,
}

pub fn search(query: &str) -> Result<Vec<Search>, reqwest::Error> {
    let base = "https://api.wiseoldman.net/players/search";
    let url = format!("{}?username={}", base, query);
    let body: Vec<Search> = reqwest::blocking::get(&url)?.json()?;
    Ok(body)
}
#[derive(Deserialize, Debug)]
pub struct Skill {
    pub rank: i64,
    pub experience: i64,
    pub ehp: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub created_at: String,
    pub imported_at: Option<String>,
    pub overall: Skill,
    pub attack: Skill,
    pub defence: Skill,
    pub strength: Skill,
    pub hitpoints: Skill,
    pub ranged: Skill,
    pub prayer: Skill,
    pub magic: Skill,
    pub cooking: Skill,
    pub woodcutting: Skill,
    pub fletching: Skill,
    pub fishing: Skill,
    pub firemaking: Skill,
    pub crafting: Skill,
    pub smithing: Skill,
    pub mining: Skill,
    pub herblore: Skill,
    pub agility: Skill,
    pub thieving: Skill,
    pub slayer: Skill,
    pub farming: Skill,
    pub runecrafting: Skill,
    pub hunter: Skill,
    pub construction: Skill,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub exp: i32,
    pub id: i32,
    pub username: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub build: String,
    pub country: Option<String>,
    pub flagged: bool,
    pub ehp: f64,
    pub ehb: f64,
    #[serde(rename = "ttm")]
    pub time_to_max: f64,
    #[serde(rename = "tt200m")]
    pub time_to_200m: f64,
    pub last_imported_at: Option<String>,
    pub last_changed_at: Option<String>,
    pub registered_at: Option<String>,
    pub updated_at: Option<String>,
    pub combat_level: i32,
    pub latest_snapshot: Snapshot,
}

pub fn get_player(username: &str) -> Result<Player, reqwest::Error> {
    let base = "https://api.wiseoldman.net/players/username";
    let url = format!("{}/{}", base, username);
    let body: Player = reqwest::blocking::get(&url)?.json()?;
    Ok(body)
}
