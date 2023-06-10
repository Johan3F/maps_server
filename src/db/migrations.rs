use anyhow::bail;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// this embeddes the migrations into the application binary
// the migration path is releative to the `CARGO_MANIFEST_DIR`
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations/");

pub async fn run_migration(pool: Pool<Manager<PgConnection>>) -> anyhow::Result<()> {
    let conn = pool.get().await?;
    let result = conn
        .interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await;
    if result.is_err() {
        bail!(
            "unable to run migrations: {}",
            result.err().unwrap().to_string()
        );
    }

    Ok(())
}
