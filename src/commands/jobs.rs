use futures::stream::{self, StreamExt};
use poise::serenity_prelude as serenity;
use std::borrow::Cow;

use crate::utils::checks::is_not_registrant_check;
use crate::utils::permissions::has_role;
use crate::{Context, Error};

#[derive(Debug, poise::ChoiceParameter)]
enum JobChoice {
    Paladin,
    Warrior,
    #[name = "Dark Knight"]
    DarkKnight,
    Gunbreaker,
    #[name = "White Mage"]
    WhiteMage,
    Scholar,
    Astrologian,
    Sage,
    Monk,
    Dragoon,
    Ninja,
    Samurai,
    Reaper,
    Viper,
    Bard,
    Machinist,
    Dancer,
    #[name = "Black Mage"]
    BlackMage,
    Summoner,
    #[name = "Red Mage"]
    RedMage,
    Pictomancer,
    #[name = "Blue Mage"]
    BlueMage,
}

/// Select your job roles.
#[poise::command(
    slash_command,
    category = "Jobs",
    check = "is_not_registrant_check",
    ephemeral
)]
pub async fn job(ctx: Context<'_>, job: JobChoice) -> Result<(), Error> {
    let member = ctx
        .author_member()
        .await
        .map(|cow_member| match cow_member {
            Cow::Owned(member) => member,
            Cow::Borrowed(member) => member.clone(),
        })
        .ok_or_else(|| Error::from("Failed to get member"))?;

    /// Returns true if the member has the role added
    /// or false if the member has the role removed
    async fn change_role_status(
        ctx: Context<'_>,
        role_id: u64,
        member: serenity::Member,
    ) -> Result<bool, Error> {
        if has_role(ctx, role_id, Some(member.clone())).await? {
            member.remove_role(ctx, role_id).await?;
            return Ok(false);
        }

        member.add_role(ctx, role_id).await?;
        Ok(true)
    }

    let role_id = match job {
        JobChoice::Paladin => ctx.data().config.roles.jobs.paladin_role_id,
        JobChoice::Warrior => ctx.data().config.roles.jobs.warrior_role_id,
        JobChoice::DarkKnight => ctx.data().config.roles.jobs.dark_knight_role_id,
        JobChoice::Gunbreaker => ctx.data().config.roles.jobs.gunbreaker_role_id,
        JobChoice::WhiteMage => ctx.data().config.roles.jobs.white_mage_role_id,
        JobChoice::Scholar => ctx.data().config.roles.jobs.scholar_role_id,
        JobChoice::Astrologian => ctx.data().config.roles.jobs.astrologian_role_id,
        JobChoice::Sage => ctx.data().config.roles.jobs.sage_role_id,
        JobChoice::Monk => ctx.data().config.roles.jobs.monk_role_id,
        JobChoice::Dragoon => ctx.data().config.roles.jobs.dragoon_role_id,
        JobChoice::Ninja => ctx.data().config.roles.jobs.ninja_role_id,
        JobChoice::Samurai => ctx.data().config.roles.jobs.samurai_role_id,
        JobChoice::Reaper => ctx.data().config.roles.jobs.reaper_role_id,
        JobChoice::Viper => ctx.data().config.roles.jobs.viper_role_id,
        JobChoice::Bard => ctx.data().config.roles.jobs.bard_role_id,
        JobChoice::Machinist => ctx.data().config.roles.jobs.machinist_role_id,
        JobChoice::Dancer => ctx.data().config.roles.jobs.dancer_role_id,
        JobChoice::BlackMage => ctx.data().config.roles.jobs.black_mage_role_id,
        JobChoice::Summoner => ctx.data().config.roles.jobs.summoner_role_id,
        JobChoice::RedMage => ctx.data().config.roles.jobs.red_mage_role_id,
        JobChoice::Pictomancer => ctx.data().config.roles.jobs.pictomancer_role_id,
        JobChoice::BlueMage => ctx.data().config.roles.jobs.blue_mage_role_id,
    };

    let status = change_role_status(ctx, role_id, member).await?;
    if status {
        ctx.reply(format!("Added role <@&{}>", role_id)).await?;
    } else {
        ctx.reply(format!("Removed role <@&{}>", role_id)).await?;
    }

    Ok(())
}

// TODO: Actually make this way work
/// Select your job roles.
#[poise::command(
    slash_command,
    category = "Jobs",
    check = "is_not_registrant_check",
    ephemeral
)]
pub async fn _jobs(ctx: Context<'_>) -> Result<(), Error> {
    let message = poise::CreateReply::default()
        .ephemeral(true)
        .content("Select the jobs that you play.")
        .components(vec![_build_job_select_menu(ctx).await]);

    ctx.send(message).await?;

    Ok(())
}

// TODO: Just... please fix this...
async fn _build_job_select_menu(ctx: Context<'_>) -> serenity::builder::CreateActionRow {
    let job_roles = ctx.data().config.roles.jobs.to_vec();
    let options: Vec<serenity::CreateSelectMenuOption> = stream::iter(job_roles)
        .then(|role| async move {
            let role_id = serenity::RoleId::from(role);
            let label = if let Some(guild) = ctx.guild() {
                if let Some(role) = guild.roles.get(&role_id) {
                    role.name.clone()
                } else {
                    "ERROR".to_string()
                }
            } else {
                "ERROR".to_string()
            };
            let option = serenity::CreateSelectMenuOption::new(label, role.to_string());
            option.default_selection(has_role(ctx, role, None).await.unwrap())
        })
        .collect()
        .await;

    let select_menu = serenity::CreateSelectMenu::new(
        "job_select_menu",
        serenity::CreateSelectMenuKind::String { options },
    );

    serenity::CreateActionRow::SelectMenu(select_menu)
}
