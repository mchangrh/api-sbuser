use axum::{
    Json, extract::{Path, Request, State},
    middleware::Next,
    http::StatusCode,
    response::IntoResponse
};
use sqlx::SqlitePool;

use crate::db;

pub async fn health() -> &'static str {
    "ok"
}

#[derive(Clone)]
#[must_use]
pub struct AppState {
    pub pool: SqlitePool,
    pub auth: String,
    pub lock_auth: String,
    pub vip_auth: String,
}

fn error_response(status: StatusCode, message: &str) -> axum::response::Response {
    (status, message.to_string()).into_response()
}

pub async fn lookup_by_discord(
    State(state): State<AppState>,
    Path(discord_id): Path<i64>,
) -> axum::response::Response {
    let result = db::user_by_discord(&state.pool, discord_id).await;
    match result {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

pub async fn lookup_by_sbid(
    State(state): State<AppState>,
    Path(sbid): Path<String>,
) -> axum::response::Response {
    if !crate::models::validate_sbid(&sbid) {
        return error_response(StatusCode::BAD_REQUEST, "Invalid sbid format");
    }
    let result = db::user_by_sbid(&state.pool, &sbid).await;
    match result {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

pub async fn upsert_user(
    State(state): State<AppState>,
    Path(( discord_id, sbid)): Path<(i64, String)>,
) -> axum::response::Response {
    if !crate::models::validate_sbid(&sbid) {
        return error_response(StatusCode::BAD_REQUEST, "Invalid sbid format");
    }
    let result = db::upsert_user(&state.pool, &sbid, discord_id).await;
    match result {
        Ok(_) => (StatusCode::CREATED).into_response(),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(discord_id): Path<i64>,
) -> axum::response::Response {
    let result = db::delete_user(&state.pool, discord_id).await;
    match result {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

pub async fn lock_user(
    State(state): State<AppState>,
    Path((sbid, auth)): Path<(String, String)>,
) -> axum::response::Response {
    if auth != state.lock_auth {
        return error_response(StatusCode::UNAUTHORIZED, "Invalid lock auth");
    }
    if !crate::models::validate_sbid(&sbid) {
        return error_response(StatusCode::BAD_REQUEST, "Invalid sbid format");
    }
    let result = db::lock_user(&state.pool, &sbid).await;
    match result {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => error_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
    }
}

pub async fn vip_auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<axum::response::Response, StatusCode> {
    let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());
    if let Some(auth) = auth_header
        && auth == state.vip_auth
    {
        return Ok(next.run(req).await);
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<axum::response::Response, StatusCode> {
    let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());
    if let Some(auth) = auth_header
        && auth == state.auth
    {
        return Ok(next.run(req).await);
    }
    Err(StatusCode::UNAUTHORIZED)
}