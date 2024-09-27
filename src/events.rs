pub mod members;
pub mod reactions;

use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use serenity::{Context, FullEvent};

use crate::{Data, Error};

pub async fn on_event(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            println!("{} is connected!", data_about_bot.user.name)
        }
        FullEvent::GuildMemberAddition { new_member } => {
            members::add_event(ctx, data, new_member).await?
        }
        FullEvent::GuildMemberRemoval { user, .. } => {
            members::remove_event(ctx, data, user).await?
        }
        FullEvent::ReactionAdd { add_reaction } => {
            reactions::add_event(ctx, data, add_reaction).await?
        }
        FullEvent::ReactionRemove { removed_reaction } => {
            reactions::remove_event(ctx, data, removed_reaction).await?
        }
        _ => (),
    }
    Ok(())
}
