//! Database connection and queries

use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

/// Create database connection pool
pub async fn create_pool(database_url: &str) -> sqlx::Result<PgPool> {
    PgPool::connect(database_url).await
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

#[derive(Debug, Clone, FromRow)]
pub struct AnalysisRow {
    pub id: Uuid,
    pub image_url: String,
    pub ocr_text: Option<String>,
    pub confirmed_text: Option<String>,
    pub ocr_status: String,
    pub llm_status: String,
    pub ocr_completed_at: Option<DateTime<Utc>>,
    pub health_score: Option<i32>,
    pub result: Option<Value>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn insert_analysis(
    pool: &PgPool,
    image_url: &str,
    user_id: Option<Uuid>,
) -> sqlx::Result<Uuid> {
    let row = sqlx::query(
        r#"
        INSERT INTO analyses (image_url, status, ocr_status, llm_status, user_id)
        VALUES ($1, 'ocr_pending', 'pending', 'pending', $2)
        RETURNING id
        "#,
    )
    .bind(image_url)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    row.try_get::<Uuid, _>("id")
}

pub async fn attach_user_to_analysis(pool: &PgPool, id: Uuid, user_id: Uuid) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE analyses
        SET user_id = COALESCE(user_id, $2),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_ocr_status(
    pool: &PgPool,
    id: Uuid,
    ocr_status: &str,
    status: &str,
    error_message: Option<String>,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE analyses
        SET ocr_status = $2,
            status = $3,
            error_message = $4,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(ocr_status)
    .bind(status)
    .bind(error_message)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn save_ocr_result(
    pool: &PgPool,
    id: Uuid,
    text: &str,
    status: &str,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE analyses
        SET ocr_text = $2,
            ocr_status = 'completed',
            status = $3,
            ocr_completed_at = NOW(),
            error_message = NULL,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(text)
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_confirmed_text(
    pool: &PgPool,
    id: Uuid,
    text: &str,
    status: &str,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE analyses
        SET confirmed_text = $2,
            llm_status = 'pending',
            status = $3,
            error_message = NULL,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(text)
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_llm_status(
    pool: &PgPool,
    id: Uuid,
    llm_status: &str,
    status: &str,
    error_message: Option<String>,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE analyses
        SET llm_status = $2,
            status = $3,
            error_message = $4,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(llm_status)
    .bind(status)
    .bind(error_message)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_analysis_result(
    pool: &PgPool,
    id: Uuid,
    status: &str,
    health_score: Option<i32>,
    result: Option<Value>,
    error_message: Option<String>,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE analyses
        SET status = $2,
            llm_status = CASE
                WHEN $2 = 'completed' THEN 'completed'
                WHEN $2 = 'failed' THEN 'failed'
                ELSE llm_status
            END,
            health_score = $3,
            result = $4,
            error_message = $5,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(status)
    .bind(health_score)
    .bind(result)
    .bind(error_message)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_analysis(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<AnalysisRow>> {
    let row = sqlx::query_as::<_, AnalysisRow>(
        r#"
        SELECT id,
               image_url,
               ocr_text,
               confirmed_text,
               ocr_status,
               llm_status,
               ocr_completed_at,
               health_score,
               result,
               status,
               error_message,
               created_at,
               updated_at
        FROM analyses
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn list_history(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> sqlx::Result<(i64, Vec<AnalysisRow>)> {
    let total: i64 = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) as count
        FROM analyses
        WHERE status = 'completed'
        "#,
    )
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, AnalysisRow>(
        r#"
        SELECT id,
               image_url,
               ocr_text,
               confirmed_text,
               ocr_status,
               llm_status,
               ocr_completed_at,
               health_score,
               result,
               status,
               error_message,
               created_at,
               updated_at
        FROM analyses
        WHERE status = 'completed'
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((total, rows))
}

pub async fn list_user_history(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> sqlx::Result<(i64, Vec<AnalysisRow>)> {
    let total: i64 = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) as count
        FROM analyses
        WHERE user_id = $1 AND status = 'completed'
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, AnalysisRow>(
        r#"
        SELECT id,
               image_url,
               ocr_text,
               confirmed_text,
               ocr_status,
               llm_status,
               ocr_completed_at,
               health_score,
               result,
               status,
               error_message,
               created_at,
               updated_at
        FROM analyses
        WHERE user_id = $1 AND status = 'completed'
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((total, rows))
}

pub async fn delete_user_history(pool: &PgPool, user_id: Uuid, id: Uuid) -> sqlx::Result<u64> {
    let result = sqlx::query(
        r#"
        DELETE FROM analyses
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn delete_user_histories(
    pool: &PgPool,
    user_id: Uuid,
    ids: &[Uuid],
) -> sqlx::Result<u64> {
    let result = sqlx::query(
        r#"
        DELETE FROM analyses
        WHERE user_id = $1 AND id = ANY($2)
        "#,
    )
    .bind(user_id)
    .bind(ids)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn migrate_user_histories(
    pool: &PgPool,
    user_id: Uuid,
    ids: &[Uuid],
) -> sqlx::Result<u64> {
    let result = sqlx::query(
        r#"
        UPDATE analyses
        SET user_id = $1,
            updated_at = NOW()
        WHERE user_id IS NULL
          AND id = ANY($2)
        "#,
    )
    .bind(user_id)
    .bind(ids)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn prune_user_history(
    pool: &PgPool,
    user_id: Uuid,
    delete_count: i64,
) -> sqlx::Result<u64> {
    if delete_count <= 0 {
        return Ok(0);
    }

    let result = sqlx::query(
        r#"
        DELETE FROM analyses
        WHERE id IN (
            SELECT id
            FROM analyses
            WHERE user_id = $1
            ORDER BY created_at ASC
            LIMIT $2
        )
        "#,
    )
    .bind(user_id)
    .bind(delete_count)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

#[derive(Debug, Clone, FromRow)]
pub struct CommunityPostListRow {
    pub id: Uuid,
    pub summary_text: String,
    pub health_score: i32,
    pub card_image_url: Option<String>,
    pub author_label: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CommunityPostDetailRow {
    pub id: Uuid,
    pub summary_text: String,
    pub health_score: i32,
    pub ingredients_raw: String,
    pub card_payload: Value,
    pub card_image_url: Option<String>,
    pub author_label: String,
    pub created_at: DateTime<Utc>,
}

pub async fn insert_community_post(
    pool: &PgPool,
    author_type: &str,
    user_id: Option<Uuid>,
    share_token_hash: Option<&str>,
    summary_text: &str,
    health_score: i32,
    ingredients_raw: &str,
    card_payload: Value,
    card_image_url: Option<&str>,
    source_analysis_id: Option<Uuid>,
) -> sqlx::Result<(Uuid, DateTime<Utc>, Option<String>)> {
    let row = sqlx::query(
        r#"
        INSERT INTO community_posts (
            author_type,
            user_id,
            share_token_hash,
            summary_text,
            health_score,
            ingredients_raw,
            card_payload,
            card_image_url,
            source_analysis_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, created_at, card_image_url
        "#,
    )
    .bind(author_type)
    .bind(user_id)
    .bind(share_token_hash)
    .bind(summary_text)
    .bind(health_score)
    .bind(ingredients_raw)
    .bind(card_payload)
    .bind(card_image_url)
    .bind(source_analysis_id)
    .fetch_one(pool)
    .await?;

    let id = row.try_get::<Uuid, _>("id")?;
    let created_at = row.try_get::<DateTime<Utc>, _>("created_at")?;
    let card_image_url = row.try_get::<Option<String>, _>("card_image_url")?;
    Ok((id, created_at, card_image_url))
}

pub async fn list_community_posts(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> sqlx::Result<(i64, Vec<CommunityPostListRow>)> {
    let total: i64 = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) as count
        FROM community_posts
        "#,
    )
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, CommunityPostListRow>(
        r#"
        SELECT p.id,
               p.summary_text,
               p.health_score,
               p.card_image_url,
               p.created_at,
               CASE
                   WHEN p.author_type = 'anonymous' THEN '匿名用户'
                   ELSE COALESCE(u.username, '用户')
               END AS author_label
        FROM community_posts p
        LEFT JOIN users u ON p.user_id = u.id
        ORDER BY p.created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((total, rows))
}

pub async fn get_community_post(
    pool: &PgPool,
    id: Uuid,
) -> sqlx::Result<Option<CommunityPostDetailRow>> {
    let row = sqlx::query_as::<_, CommunityPostDetailRow>(
        r#"
        SELECT p.id,
               p.summary_text,
               p.health_score,
               p.ingredients_raw,
               p.card_payload,
               p.card_image_url,
               p.created_at,
               CASE
                   WHEN p.author_type = 'anonymous' THEN '匿名用户'
                   ELSE COALESCE(u.username, '用户')
               END AS author_label
        FROM community_posts p
        LEFT JOIN users u ON p.user_id = u.id
        WHERE p.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn delete_community_post_by_user(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
) -> sqlx::Result<u64> {
    let result = sqlx::query(
        r#"
        DELETE FROM community_posts
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn delete_community_post_by_token(
    pool: &PgPool,
    id: Uuid,
    share_token_hash: &str,
) -> sqlx::Result<u64> {
    let result = sqlx::query(
        r#"
        DELETE FROM community_posts
        WHERE id = $1 AND share_token_hash = $2
        "#,
    )
    .bind(id)
    .bind(share_token_hash)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

#[allow(dead_code)]
#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub username: Option<String>,
    pub username_normalized: Option<String>,
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserPreferencesRow {
    pub preferences: Value,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_user_preferences(
    pool: &PgPool,
    user_id: Uuid,
) -> sqlx::Result<Option<UserPreferencesRow>> {
    let row = sqlx::query_as::<_, UserPreferencesRow>(
        r#"
        SELECT preferences,
               updated_at
        FROM user_preferences
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn upsert_user_preferences(
    pool: &PgPool,
    user_id: Uuid,
    preferences: &Value,
) -> sqlx::Result<UserPreferencesRow> {
    let row = sqlx::query_as::<_, UserPreferencesRow>(
        r#"
        INSERT INTO user_preferences (user_id, preferences)
        VALUES ($1, $2)
        ON CONFLICT (user_id)
        DO UPDATE SET preferences = $2, updated_at = NOW()
        RETURNING preferences, updated_at
        "#,
    )
    .bind(user_id)
    .bind(preferences)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Option<UserRow>> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id,
               username,
               username_normalized,
               password_hash,
               created_at,
               updated_at,
               last_login_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn get_user_by_username(
    pool: &PgPool,
    username_normalized: &str,
) -> sqlx::Result<Option<UserRow>> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id,
               username,
               username_normalized,
               password_hash,
               created_at,
               updated_at,
               last_login_at
        FROM users
        WHERE username_normalized = $1
        "#,
    )
    .bind(username_normalized)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn create_user_with_password(
    pool: &PgPool,
    username: &str,
    username_normalized: &str,
    password_hash: &str,
) -> sqlx::Result<UserRow> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"
        INSERT INTO users (username, username_normalized, password_hash, last_login_at)
        VALUES ($1, $2, $3, NOW())
        RETURNING id,
                  username,
                  username_normalized,
                  password_hash,
                  created_at,
                  updated_at,
                  last_login_at
        "#,
    )
    .bind(username)
    .bind(username_normalized)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_user_last_login(pool: &PgPool, user_id: Uuid) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        UPDATE users
        SET last_login_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn count_user_analyses(pool: &PgPool, user_id: Uuid) -> sqlx::Result<i64> {
    let total = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*) as count
        FROM analyses
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(total)
}

pub async fn ensure_user_preferences(pool: &PgPool, user_id: Uuid) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO user_preferences (user_id, preferences)
        VALUES ($1, '{}'::jsonb)
        ON CONFLICT (user_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}
