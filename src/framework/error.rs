use crate::types::{Data, Error};
use poise::FrameworkError;
use tracing::error;

/// Global Framework Error Handler
pub async fn on_framework_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::Setup { error, .. } => {
            error!("Initialization error: {:?}", error);
        }

        FrameworkError::Command { error, ctx, .. } => {
            error!("A mistake in the command: {:?}", error);
            let _ = ctx.say("An error occurred while executing the command").await;
        }

        other => {
            if let Err(e) = poise::builtins::on_error(other).await {
                error!("Processing error: {:?}", e);
            }
        }
    }
}
