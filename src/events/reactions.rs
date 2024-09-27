use poise::serenity_prelude as serenity;

use crate::{Data, Error};

// TODO:
// This entire functionality needs to be implemented.
// We need a way to associate roles with reactions, and messages with the feature in general.

// NOTE: This is not currently used.

pub async fn add_event(
    _ctx: &serenity::Context,
    _data: &Data,
    _reaction: &serenity::Reaction,
) -> Result<(), Error> {
    Ok(())
}

pub async fn remove_event(
    _ctx: &serenity::Context,
    _data: &Data,
    _reaction: &serenity::Reaction,
) -> Result<(), Error> {
    Ok(())
}
