use crate::leetcode::{models::ProblemDescription, query::{get_daily_problem_slug, get_problem_description}, views::problem_view};

use reqwest::Error;
use serenity::all::{CommandInteraction, Context, CreateCommand};

pub async fn exec(ctx: Context, command: CommandInteraction) -> Result<(), Error> {
    tracing::info!("`daily` command executed by `{} ({})`", command.user.name, command.user.id);

    let question_info = run().await;
    let builder = match question_info {
        Ok(question_info) => {
            problem_view(Some("Daily LeetCode Problem"), Some(question_info)) 
        }
        Err(why) => {
            tracing::error!("problem error encountered: {}", why);
            problem_view(None, None)
        }
    };

    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
    
    Ok(())
}

pub async fn run() -> Result<ProblemDescription, Error> {
    let client = reqwest::Client::new();
    let problem_slug = get_daily_problem_slug(&client).await?;
    let question_info = get_problem_description(&client, problem_slug).await?;

    Ok(question_info)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("daily").description("Get information on the daily leetcode problem")
}