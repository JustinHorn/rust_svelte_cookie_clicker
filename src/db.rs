use crate::{data::*, error, error::Error::*, DBCon, DBPool};
use chrono::prelude::*;
use mobc::Pool;
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::env;
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls, Row};

type Result<T> = std::result::Result<T, error::Error>;

const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;
const INIT_SQL: &str = "./db.sql";
const TABLE: &str = "player";
const SELECT_FIELDS: &str = "id, name, created_at, count";

pub async fn init_db(db_pool: &DBPool) -> Result<()> {
    let init_file = fs::read_to_string(INIT_SQL)?;
    let con = get_db_con(db_pool).await?;
    con.batch_execute(init_file.as_str())
        .await
        .map_err(DBInitError)?;
    Ok(())
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon> {
    db_pool.get().await.map_err(DBPoolError)
}

pub fn create_pool() -> std::result::Result<DBPool, mobc::Error<Error>> {
    let postgres_url = env::var("POSTGRES_URL").unwrap();
    println!("Connecting to {}", postgres_url);

    let config = Config::from_str(postgres_url.as_str())?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn fetch_players(db_pool: &DBPool, search: Option<String>) -> Result<Vec<Player>> {
    let con = get_db_con(db_pool).await?;
    let where_clause = match search {
        Some(_) => "WHERE name like $1",
        None => "",
    };
    let query = format!(
        "SELECT {} FROM {} {} ORDER BY created_at DESC",
        SELECT_FIELDS, TABLE, where_clause
    );
    let q = match search {
        Some(v) => con.query(query.as_str(), &[&v]).await,
        None => con.query(query.as_str(), &[]).await,
    };
    let rows = q.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_player(&r)).collect())
}

pub async fn fetch_player_by_id(db_pool: &DBPool, id: i32) -> Result<Player> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = $1", SELECT_FIELDS, TABLE);
    let row = con
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;
    println!("Fetched player {}", id);
    Ok(row_to_player(&row))
}

pub async fn fetch_player_by_name(db_pool: &DBPool, name: String) -> Result<Player> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE name = $1", SELECT_FIELDS, TABLE);
    let row = con
        .query_one(query.as_str(), &[&name])
        .await
        .map_err(DBQueryError)?;
    println!("Fetched player {}", name);
    Ok(row_to_player(&row))
}

pub async fn create_player(db_pool: &DBPool, body: PlayerRequest) -> Result<Player> {
    match body.name.parse::<i32>() {
        Ok(_) => return Err(PlayerNameCantBeANumberError()),
        Err(_) => {
            let con = get_db_con(db_pool).await?;
            let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
            let row = con
                .query_one(query.as_str(), &[&body.name])
                .await
                .map_err(DBQueryError)?;
            Ok(row_to_player(&row))
        }
    }
}

pub async fn update_player_by_name(
    db_pool: &DBPool,
    name: String,
    body: PlayerUpdateRequest,
) -> Result<Player> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
        "UPDATE {} SET count = $1 WHERE name = $2 RETURNING *",
        TABLE
    );
    let row = con
        .query_one(query.as_str(), &[&body.count, &name])
        .await
        .map_err(DBQueryError)?;
    println!("Updated player {} count to {}", name, body.count);
    Ok(row_to_player(&row))
}

pub async fn update_player_by_id(
    db_pool: &DBPool,
    id: i32,
    body: PlayerUpdateRequest,
) -> Result<Player> {
    let con = get_db_con(db_pool).await?;
    let query = format!("UPDATE {} SET count = $1 WHERE id = $2 RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.count, &id])
        .await
        .map_err(DBQueryError)?;
    println!("Updated player {} count to {}", id, body.count);
    Ok(row_to_player(&row))
}

pub async fn delete_player(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}

fn row_to_player(row: &Row) -> Player {
    let id: i32 = row.get(0);
    let name: String = row.get(1);
    let created_at: DateTime<Utc> = row.get(2);
    let count: i32 = row.get(3);
    Player {
        id,
        name,
        created_at,
        count,
    }
}
