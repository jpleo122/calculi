use clap::Parser;
use calculi_api::config::Config;
use calculi_api::api::the_fun_begins;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::parse();

    the_fun_begins(config).await?;

    Ok(())
}