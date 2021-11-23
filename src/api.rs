// pub fn get(path: &str) -> Result<(), reqwest::Error> {
//   let base = "https://api.wiseoldman.net";
//   let url = format!("{}/{}", base, path);

// let body = reqwest::blocking::get(url)?.json()?;
//   return body;
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

// {
//   "exp":32705041,
//   "id":40943,
//   "username":"poweron fliz",
//   "displayName":"PowerOn fliz",
//   "type":"regular",
//   "build":"main",
//   "country":null,
//   "flagged":false,
//   "ehp":183.82398,
//   "ehb":0,
//   "ttm":1109.93197,
//   "tt200m":14778.62707,
//   "lastImportedAt":"2020-11-04T17:04:06.225Z",
//   "lastChangedAt":null,
//   "registeredAt":"2020-09-11T14:30:15.422Z",
//   "updatedAt":"2020-11-04T17:04:06.225Z"}
