use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
    pub readme: String,
}