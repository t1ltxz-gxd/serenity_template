use crate::types::{Data, Error, MIGRATOR};
use poise::serenity_prelude as serenity;
use sqlx::SqlitePool;

/// Setup hook - called once at startup
pub async fn setup_hook(
    ctx: &serenity::Context,
    _ready: &serenity::Ready,
    framework: &poise::Framework<Data, Error>,
) -> Result<Data, Error> {
    // Регистрируем slash-команды глобально
    poise::builtins::register_globally(ctx, &framework.options().commands).await?;

    // Инициализация SQLx
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db = SqlitePool::connect(&database_url).await?;

    MIGRATOR.run(&db).await?;

    tracing::info!("Framework is ready with DB!");

    tracing::info!("Framework is ready!");

    // Возвращаем объект Data, если там хранятся настройки/клиенты БД и т.п.
    Ok(Data{db})
}
