use crate::types::{Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    description_localized("en-US", "Check bot latency")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    // The simplest database check
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&ctx.data().db)
        .await?;
    tracing::info!("Database check returned: {}", row.0);

    ctx.say("Pong! 🏓").await?;
    Ok(())
}
