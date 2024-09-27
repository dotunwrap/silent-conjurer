use crate::utils::permissions;
use crate::{Context, Error};

pub async fn is_master_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_master(ctx, None).await
}

pub async fn is_commander_or_above_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_commander_or_above(ctx, None).await
}

pub async fn is_sergeant_or_above_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_sergeant_or_above(ctx, None).await
}

pub async fn is_officer_or_above_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_officer_or_above(ctx, None).await
}

pub async fn is_veteran_or_above_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_veteran_or_above(ctx, None).await
}

pub async fn is_member_or_above_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_member_or_above(ctx, None).await
}

pub async fn is_recruit_or_above_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_recruit_or_above(ctx, None).await
}

pub async fn is_guest_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_guest(ctx, None).await
}

pub async fn is_not_registrant_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_registrant(ctx, None)
        .await
        .map(|is_registrant| !is_registrant)
}

pub async fn is_registrant_check(ctx: Context<'_>) -> Result<bool, Error> {
    permissions::is_registrant(ctx, None).await
}
