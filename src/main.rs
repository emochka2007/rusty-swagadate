use crate::bot::SwagaBot;
use crate::pg::establish_connection;

mod bot;
mod match_engine;
mod pg;
mod profile;
mod profile_activities;
mod profile_view;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    log::info!("Starting bot");

    SwagaBot::dispatcher().await;

    Ok(())
}
