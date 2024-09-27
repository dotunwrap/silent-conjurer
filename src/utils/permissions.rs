use poise::serenity_prelude as serenity;
use std::{borrow::Cow, future::Future};

use crate::{Context, Error};

pub async fn is_master(ctx: Context<'_>, member: Option<serenity::Member>) -> Result<bool, Error> {
    has_role(ctx, ctx.data().config.roles.master_role_id, member).await
}

pub async fn is_commander_or_above(
    ctx: Context<'_>,
    member: Option<serenity::Member>,
) -> Result<bool, Error> {
    has_role_or_above(
        ctx,
        ctx.data().config.roles.commander_role_id,
        is_master,
        member,
    )
    .await
}

pub async fn is_sergeant_or_above(
    ctx: Context<'_>,
    member: Option<serenity::Member>,
) -> Result<bool, Error> {
    has_role_or_above(
        ctx,
        ctx.data().config.roles.sergeant_role_id,
        is_commander_or_above,
        member,
    )
    .await
}

pub async fn is_officer_or_above(
    ctx: Context<'_>,
    member: Option<serenity::Member>,
) -> Result<bool, Error> {
    has_role_or_above(
        ctx,
        ctx.data().config.roles.officer_role_id,
        is_commander_or_above,
        member,
    )
    .await
}

pub async fn is_veteran_or_above(
    ctx: Context<'_>,
    member: Option<serenity::Member>,
) -> Result<bool, Error> {
    has_role_or_above(
        ctx,
        ctx.data().config.roles.veteran_role_id,
        is_officer_or_above,
        member,
    )
    .await
}

pub async fn is_member_or_above(
    ctx: Context<'_>,
    member: Option<serenity::Member>,
) -> Result<bool, Error> {
    has_role_or_above(
        ctx,
        ctx.data().config.roles.member_role_id,
        is_officer_or_above,
        member,
    )
    .await
}

pub async fn is_recruit_or_above(
    ctx: Context<'_>,
    member: Option<serenity::Member>,
) -> Result<bool, Error> {
    has_role_or_above(
        ctx,
        ctx.data().config.roles.recruit_role_id,
        is_member_or_above,
        member,
    )
    .await
}

pub async fn is_guest(ctx: Context<'_>, member: Option<serenity::Member>) -> Result<bool, Error> {
    has_role(ctx, ctx.data().config.roles.guest_role_id, member).await
}

pub async fn is_registrant(
    ctx: Context<'_>,
    member: Option<serenity::Member>,
) -> Result<bool, Error> {
    has_role(ctx, ctx.data().config.roles.registrant_role_id, member).await
}

pub async fn has_role_or_above<'a, F, Fut>(
    ctx: Context<'a>,
    role_id: u64,
    next_check: F,
    member: Option<serenity::Member>,
) -> Result<bool, Error>
where
    F: for<'b> Fn(Context<'a>, Option<serenity::Member>) -> Fut,
    Fut: Future<Output = Result<bool, Error>> + 'a,
{
    match has_role(ctx, role_id, member.clone()).await {
        Ok(has_role) if has_role => Ok(true),
        Ok(_) => next_check(ctx, member).await,
        Err(error) => Err(error),
    }
}

pub async fn has_role(
    ctx: Context<'_>,
    role_id: u64,
    member: Option<serenity::Member>,
) -> Result<bool, Error> {
    let member = if let Some(member) = member {
        Ok(member)
    } else {
        ctx.author_member()
            .await
            .map(|cow_member| match cow_member {
                Cow::Owned(member) => member,
                Cow::Borrowed(member) => member.clone(),
            })
            .ok_or_else(|| Error::from("Failed to get member"))
    };

    match member {
        Ok(member) => Ok(member.roles.iter().any(|id| *id == role_id)),
        Err(error) => Err(error),
    }
}
