use crate::{Data, Error};

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to build framework: {:?}", error)
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!(
                "Command '{}' returned an error: {}",
                ctx.command().qualified_name,
                error
            );

            ctx.reply(error.to_string()).await.ok();
        }
        poise::FrameworkError::CommandCheckFailed { error, ctx, .. } => {
            println!(
                "Command '{}' failed check: {:?}",
                ctx.command().qualified_name,
                error
            );

            ctx.reply("You do not have permission to use this command.")
                .await
                .ok();
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Failed to call on_error: {}", e);
            }
        }
    }
}
