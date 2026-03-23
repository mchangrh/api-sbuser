mod db;
mod models;
mod routes;

use axum::Router;
use routes::AppState;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://data/sbuser.db")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    // private vip auth/ bot auth
    let auth: String = std::env::var("AUTH").unwrap_or_else(|_| "".to_string());
    // lock auth
    let lock_auth: String = std::env::var("LOCK_AUTH").unwrap_or_else(|_| "".to_string());
    // vip auth
    let vip_auth: String = std::env::var("VIP_AUTH").unwrap_or_else(|_| "".to_string());

    let state = AppState { pool, auth, lock_auth, vip_auth };

    let vip = Router::new()
        .route("/vip/lookup/discord/{discord_id}", axum::routing::get(routes::lookup_by_discord))
        .route("/vip/lookup/sbid/{sbid}", axum::routing::get(routes::lookup_by_sbid))
        .layer(axum::middleware::from_fn_with_state(state.clone(), routes::vip_auth_middleware))
        .with_state(state.clone());

    let public = Router::new()
        .route("/health", axum::routing::get(routes::health));

    let protected = Router::new()
        .route("/lookup/discord/{discord_id}", axum::routing::get(routes::lookup_by_discord))
        .route("/lookup/sbid/{sbid}", axum::routing::get(routes::lookup_by_sbid))
        .route("/upsert/{discord_id}/{sbid}", axum::routing::post(routes::upsert_user))
        .route("/delete/{discord_id}", axum::routing::delete(routes::delete_user))
        .route("/lock/{sbid}/{auth}", axum::routing::post(routes::lock_user))
        .layer(axum::middleware::from_fn_with_state(state.clone(), routes::auth_middleware))
        .with_state(state);

    let app = public.merge(protected).merge(vip);

    let addr = "0.0.0.0:3000";
    println!("Listening on http://{}", addr);
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");
    axum::serve(listener, app)
        .await?;

    Ok(())
}