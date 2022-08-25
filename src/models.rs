use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub pass: String,
    pub email: String,
}


#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub pass: String,
}


#[derive(Debug, Deserialize)]
pub struct CreateEntryPayload {
    pub meters: f32,
}


#[derive(sqlx::FromRow, Serialize)]
pub struct MeterData {
    pub id: i64,
    pub time: String,
    pub meters: f32,
}

#[derive(Serialize)]
pub struct MeterDataList {
    pub sum: f32,
    pub entries: Vec<MeterData>,
}

#[derive(sqlx::FromRow, Clone, Serialize)]
pub struct HighscoreEntry {
    pub username: String,
    pub meters: f32,
}

#[derive(Serialize)]
pub struct HighscoreList {
    pub entries: Vec<HighscoreEntry>,
}
