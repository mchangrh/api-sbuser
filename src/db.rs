use sqlx::sqlite::SqlitePool;
use crate::models::User;

// GET
pub async fn user_by_sbid(pool: &SqlitePool, sbid: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"SELECT sbid, discord, locked FROM userid WHERE sbid = ? LIMIT 1"#,
    )
    .bind(sbid)
    .fetch_optional(pool)
    .await
}

pub async fn user_by_discord(pool: &SqlitePool, discord: i64) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"SELECT sbid, discord, locked FROM userid WHERE discord = ? LIMIT 1"#,
    )
    .bind(discord)
    .fetch_optional(pool)
    .await
}

// update or insert
pub async fn upsert_user(pool: &SqlitePool, sbid: &str, discord: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO userid (sbid, discord) VALUES (?, ?)
           ON CONFLICT(discord) DO UPDATE SET sbid = excluded.sbid
           WHERE locked = 0"#,
    )
    .bind(sbid)
    .bind(discord)
    .execute(pool)
    .await?;
    Ok(())
}

// delete
pub async fn delete_user(pool: &SqlitePool, discord: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"DELETE FROM userid WHERE discord = ?
            AND locked = 0"#,
    )
    .bind(discord)
    .execute(pool)
    .await?;
    Ok(())
}

// lock
pub async fn lock_user(pool: &SqlitePool, sbid: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE userid SET locked = 1 WHERE sbid = ?"#,
    )
    .bind(sbid)
    .execute(pool)
    .await?;
    Ok(())
}