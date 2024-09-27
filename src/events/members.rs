use poise::serenity_prelude as serenity;

use crate::{Data, Error};

pub async fn add_event(
    ctx: &serenity::Context,
    data: &Data,
    member: &serenity::Member,
) -> Result<(), Error> {
    let ret_val = match member
        .add_role(ctx, data.config.roles.registrant_role_id)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    };

    serenity::ChannelId::from(data.config.channels.registrant_channel_id).say(
        ctx,
        format!(
            "Welcome <@{}>! If you are already a member of the FC, please use /register to notify officers of your arrival. Otherwise, please use contact an online officer to discuss applying. Good luck, Warrior of Light!",
            member.user.id
        ),
    ).await?;

    ret_val
}

// Once a member leaves a guild, their member data may no longer be available.
// We will rely on the user data instead.
pub async fn remove_event(
    ctx: &serenity::Context,
    data: &Data,
    user: &serenity::User,
) -> Result<(), Error> {
    match serenity::ChannelId::from(data.config.channels.officers_channel_id)
        .say(
            ctx,
            format!("{} ({}) has left the guild.", user.name, user.id),
        )
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
