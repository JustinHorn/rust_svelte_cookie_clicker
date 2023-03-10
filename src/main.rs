use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};

mod data;
mod db;
mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_pool = db::create_pool().expect("database pool can be created");

    db::init_db(&db_pool)
        .await
        .expect("database can be initialized");

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handler::health_handler);

    let player = warp::path("player");
    let player_routes = player
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db_pool.clone()))
        .and_then(handler::get_player_handler)
        .or(warp::path("players")
            .and(warp::get())
            .and(warp::query())
            .and(with_db(db_pool.clone()))
            .and_then(handler::list_players_handler))
        .or(player
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_player_handler))
        .or(player
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_player_handler))
        .or(player
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_player_handler));

    let routes = health_route
        .or(player_routes)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec![
                    "Access-Control-Allow-Origin",
                    "Origin",
                    "Accept",
                    "X-Requested-With",
                    "Content-Type",
                ])
                .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]),
        )
        .or(warp::fs::dir("./client/build"))
        .or(warp::fs::file("./client/build/index.html"))
        .with(warp::log("api"))
        .recover(error::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
