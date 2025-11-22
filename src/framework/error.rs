use crate::types::{Data, Error};
use poise::FrameworkError;

/// Глобальный обработчик ошибок фреймворка
pub async fn on_framework_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            eprintln!("Ошибка инициализации: {:?}", error);
        }

        FrameworkError::Command { error, ctx, .. } => {
            eprintln!("Ошибка в команде: {:?}", error);
            let _ = ctx.say("Произошла ошибка при выполнении команды").await;
        }

        other => {
            if let Err(e) = poise::builtins::on_error(other).await {
                eprintln!("Ошибка обработки: {:?}", e);
            }
        }
    }
}
