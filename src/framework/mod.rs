mod setup;
mod options;
mod error;

pub use setup::setup_hook;
pub use options::framework_options;
pub use error::on_framework_error;

use crate::types::{Data, Error};
use poise::Framework;

/// Собирает полностью готовый фреймворк Poise
pub fn build_framework() -> Framework<Data, Error> {
    Framework::builder()
        .setup(|ctx, ready, framework| {
            Box::pin(setup_hook(ctx, ready, framework))
        })
        .options(framework_options())
        .build()
}
