use anyhow::Result;
use sqlx::PgPool;

use backend::config::AppConfig;
use backend::services::rules::load_items_from_path;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env()?;
    let pool = PgPool::connect(&config.database_url).await?;

    let items = load_items_from_path(&config.rules_path)?;
    let total = items.len();
    let mut tx = pool.begin().await?;

    for item in &items {
        sqlx::query(
            "INSERT INTO rules (id, name, aliases, category, risk_level, groups, description, evidence, source, enabled)\
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, TRUE)\
             ON CONFLICT (id) DO UPDATE SET \
               name = EXCLUDED.name,\
               aliases = EXCLUDED.aliases,\
               category = EXCLUDED.category,\
               risk_level = EXCLUDED.risk_level,\
               groups = EXCLUDED.groups,\
               description = EXCLUDED.description,\
               evidence = EXCLUDED.evidence,\
               source = EXCLUDED.source,\
               enabled = TRUE,\
               updated_at = NOW()",
        )
        .bind(&item.id)
        .bind(&item.name)
        .bind(&item.aliases)
        .bind(&item.category)
        .bind(&item.risk_level)
        .bind(&item.groups)
        .bind(&item.description)
        .bind(&item.evidence)
        .bind(&item.source)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    println!("Imported {} rules", total);
    Ok(())
}
