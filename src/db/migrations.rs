use anyhow::bail;
use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// this embeddes the migrations into the application binary
// the migration path is releative to the `CARGO_MANIFEST_DIR`
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/db/migrations/");

pub async fn run_migrations(pool: &Pool<Manager<PgConnection>>) -> anyhow::Result<()> {
    let conn = pool.get().await?;
    let await_result = conn
        .interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await;

    match await_result {
        Err(error) => bail!("unable to connect to the database: {}", error),
        Ok(interaction_result) => match interaction_result {
            Ok(_) => Ok(()),
            Err(error) => bail!("unable to run migrations: {}", error),
        },
    }
}
