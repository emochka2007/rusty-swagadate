use crate::bot::SwagaBot;
use crate::pg::establish_connection;

mod profile;
mod bot;
mod schema;
mod pg;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    log::info!("Starting bot");

    SwagaBot::dispatcher().await;

    Ok(())
}
