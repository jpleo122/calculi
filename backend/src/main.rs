use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate;

use calculi_api::config::Config;
use calculi_api::api::the_fun_begins;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::parse();

    env_logger::init();

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("couldn't connect to database_url")?;

    migrate!().run(&db).await?;

    the_fun_begins(config, db).await?;

    Ok(())
}