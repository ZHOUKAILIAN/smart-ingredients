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

pub async fn insert_analysis(pool: &PgPool, image_url: &str) -> sqlx::Result<Uuid> {
    let row = sqlx::query(
        r#"
        INSERT INTO analyses (image_url, status, ocr_status, llm_status)
        VALUES ($1, 'ocr_pending', 'pending', 'pending')
        RETURNING id
        "#,
    )
    .bind(image_url)
    .fetch_one(pool)
    .await?;

    Ok(row.try_get::<Uuid, _>("id")?)
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
