use poise::serenity_prelude as serenity;
use tracing::info;

pub async fn handle_message(
    _ctx: &serenity::Context,
    msg: &serenity::Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if msg.author.bot {
        return Ok(());
    }

    info!("Received message from {}: {}", msg.author.name, msg.content);

    Ok(())
}
