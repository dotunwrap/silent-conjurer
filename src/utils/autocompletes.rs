use futures::{Stream, StreamExt};
use poise::serenity_prelude as serenity;

use crate::Context;

pub async fn autocomplete_registrant_member<'a>(
    ctx: Context<'a>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let guild = match ctx.guild() {
        Some(guild) => guild.clone(),
        None => return futures::stream::empty().boxed(),
    };

    let registrants: Vec<serenity::Member> = guild
        .members
        .clone()
        .into_iter()
        .filter_map(move |(_, member)| {
            if member
                .roles
                .iter()
                .any(|id| *id == ctx.data().config.roles.registrant_role_id)
            {
                println!("Found registrant: {}", member.display_name());
                Some(member)
            } else {
                println!("Skipping non-registrant: {}", member.display_name());
                None
            }
        })
        .collect();

    futures::stream::iter(registrants)
        .filter(move |member| {
            futures::future::ready(member.display_name().to_string().starts_with(partial))
        })
        .map(|member| member.display_name().to_string())
        .boxed()
}
