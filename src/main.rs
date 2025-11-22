use poise::serenity_prelude as serenity;
use serenity_template::events;
use serenity_template::types::Error;
use serenity_template::framework;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN")?;
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = framework::build_framework();

    // Create ClientBuilder
    let builder = serenity::ClientBuilder::new(&token, intents);

    // Registering event handlers
    let builder = events::register_handlers(builder);

    // plug in the poise framework and create a Client
    let mut client = builder
        .framework(framework)
        .await?;

    client.start().await?;
    Ok(())
}
