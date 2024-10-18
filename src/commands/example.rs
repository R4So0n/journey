use crate::Context;
/*
    This is an example on how to create the command.
    Check the docs/example.md for more information.
*/

#[poise::command(prefix_command)]
pub async fn example_command(
    ctx: Context<'_>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut response = String::new();

    response = "This is an example command.".to_string();

    ctx.say(response).await?;
    Ok(())
}
