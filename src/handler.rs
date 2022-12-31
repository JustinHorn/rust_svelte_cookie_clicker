use crate::{data::*, db, error::Error::*, DBPool, Result};
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
    let db = db::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;
    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(StatusCode::OK)
}

pub async fn list_players_handler(query: SearchQuery, db_pool: DBPool) -> Result<impl Reply> {
    let players = db::fetch_players(&db_pool, query.search)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json::<Vec<_>>(
        &players.into_iter().map(|p| PlayerResponse::of(p)).collect(),
    ))
}

pub async fn create_player_handler(body: PlayerRequest, db_pool: DBPool) -> Result<impl Reply> {
    Ok(json(&PlayerResponse::of(
        db::create_player(&db_pool, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn update_player_handler(
    id: i32,
    body: PlayerUpdateRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    Ok(json(&PlayerResponse::of(
        db::update_player(&db_pool, id, body)
            .await
            .map_err(|e| reject::custom(e))?,
    )))
}

pub async fn delete_player_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    db::delete_player(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn get_player_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
    let player = db::fetch_player(&db_pool, id)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(json(&PlayerResponse::of(player)))
}
