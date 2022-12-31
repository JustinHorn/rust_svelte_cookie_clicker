use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub count: i32,
}

#[derive(Deserialize)]
pub struct PlayerRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct PlayerUpdateRequest {
    pub count: i32,
}

#[derive(Serialize)]
pub struct PlayerResponse {
    pub id: i32,
    pub name: String,
    pub count: i32,
}

impl PlayerResponse {
    pub fn of(player: Player) -> PlayerResponse {
        PlayerResponse {
            id: player.id,
            name: player.name,
            count: player.count,
        }
    }
}
