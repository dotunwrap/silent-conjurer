use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use crate::utils::checks::{is_officer_or_above_check, is_registrant_check};
use crate::utils::permissions::is_registrant;
use crate::{ApplicationContext, Context, Error};

#[derive(poise::ChoiceParameter)]
enum MemberRole {
    #[name = "Veteran"]
    Veteran,
    #[name = "Member"]
    Member,
    #[name = "Recruit"]
    Recruit,
    #[name = "Guest"]
    Guest,
}

#[derive(Debug, poise::Modal)]
#[name = "Registration Request"]
struct RegistrationModal {
    #[name = "Character Name"]
    #[min_length = 3]
    character_name: String,
    #[name = "Are you a member of the FC?"]
    member_type: String,
    #[name = "If yes, what is your rank?"]
    member_rank: Option<String>,
}

/// Register for access to the server.
#[poise::command(
    slash_command,
    category = "Registrations",
    check = "is_registrant_check"
)]
pub async fn register(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    use poise::Modal as _;

    let data = match RegistrationModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    let embed = serenity::CreateEmbed::new()
        .title("Registration Request")
        .field("Discord", ctx.author().mention().to_string(), false)
        .field("Character Name", data.character_name, false)
        .field("Are you a member of the FC?", data.member_type, false)
        .field(
            "If yes, what is your rank?",
            data.member_rank.unwrap_or_default(),
            false,
        );

    serenity::ChannelId::from(
        ctx.data()
            .config
            .channels
            .registration_notification_channel_id,
    )
    .send_message(
        ctx.http(),
        serenity::CreateMessage::new()
            .content(format!("<@&{}>", ctx.data().config.roles.officer_role_id))
            .embed(embed),
    )
    .await
    .context("Failed to send registration request")?;

    ctx.reply("Your registration request has been submitted. An officer will review it shortly.")
        .await
        .context("Failed to send registration request")?;

    Ok(())
}

/// Approves a registrant and grants access to the server.
#[poise::command(
    slash_command,
    category = "Registrations",
    check = "is_officer_or_above_check"
)]
pub async fn approve(
    ctx: Context<'_>,
    member: serenity::Member,
    role: MemberRole,
) -> Result<(), Error> {
    if !is_registrant(ctx, Some(member.clone())).await? {
        return Err(Error::from("The specified member is not a registrant."));
    }

    let role_id = match role {
        MemberRole::Veteran => ctx.data().config.roles.veteran_role_id,
        MemberRole::Member => ctx.data().config.roles.member_role_id,
        MemberRole::Recruit => ctx.data().config.roles.recruit_role_id,
        MemberRole::Guest => ctx.data().config.roles.guest_role_id,
    };

    member
        .remove_role(ctx, ctx.data().config.roles.registrant_role_id)
        .await?;
    member.add_role(ctx, role_id).await?;
    member
        .add_role(ctx, ctx.data().config.roles.jobs.divider_role_id)
        .await?;

    ctx.reply("Successfully approved registration.")
        .await
        .context("Failed to send confirmation message")?;

    serenity::ChannelId::from(ctx.data().config.channels.general_channel_id)
        .send_message(
            ctx.http(),
            serenity::CreateMessage::new().content(format!(
                "Please welcome <@{}> to Caelestis!",
                member.user.id
            )),
        )
        .await
        .context("Failed to send welcome message")?;

    Ok(())
}
