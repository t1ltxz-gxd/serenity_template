use crate::{commands, types::Data, types::Error};
use poise::PrefixFrameworkOptions;

/// Возвращает полностью готовый FrameworkOptions
pub fn framework_options() -> poise::FrameworkOptions<Data, Error> {
    poise::FrameworkOptions {
        commands: commands::all_commands(),

        prefix_options: PrefixFrameworkOptions {
            prefix: Some("!".into()),
            ignore_bots: true,
            case_insensitive_commands: true,
            ..Default::default()
        },

        on_error: |error| Box::pin(crate::framework::on_framework_error(error)),

        ..Default::default()
    }
}
