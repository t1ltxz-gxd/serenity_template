pub mod ready;
pub mod message;
pub mod error;

use poise::serenity_prelude as serenity;

/// Registers event handlers - takes ClientBuilder by value and returns it back,
/// because .event_handler consumes self -> Self.
pub fn register_handlers(
    client_builder: serenity::ClientBuilder,
) -> serenity::ClientBuilder {
    client_builder.event_handler(EventHandler)
}

/// Handler structure that handles events Serenity
pub struct EventHandler;

#[serenity::async_trait]
impl serenity::EventHandler for EventHandler {
    async fn cache_ready(&self, _ctx: serenity::Context, _guilds: Vec<serenity::GuildId>) {
        tracing::info!("Cache is ready!");
    }

    async fn message(&self, ctx: serenity::Context, msg: serenity::Message) {
        // call the public function from message.rs
        if let Err(e) = message::handle_message(&ctx, &msg).await {
            tracing::error!("Ошибка в handle_message: {:?}", e);
        }
    }

    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready) {
        // call the public function from ready.rs
        if let Err(e) = ready::handle_ready(&ctx, &ready).await {
            tracing::error!("Ошибка в handle_ready: {:?}", e);
        }
    }
}
