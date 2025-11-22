use crate::types::{Data, Error};
mod ping;

pub fn all_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        ping::ping(),
    ]
}
