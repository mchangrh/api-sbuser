use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub sbid: String,
    pub discord: i64,
    #[serde(default)]
    pub locked: bool,
}

pub fn validate_sbid(sbid: &str) -> bool {
    sbid.len() == 64 && sbid.chars().all(|c| c.is_ascii_hexdigit())
}