use crate::leetcode::{models::ProblemDescription, query::{get_problem_description, get_problem_slug_by_id}, views::problem_view};

use reqwest::Error;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue};

pub async fn exec(ctx: Context, command: CommandInteraction) -> Result<(), Error> {
    tracing::info!("`problem` command executed by `{} ({})`", command.user.name, command.user.id);
    
    let question_info = run(&command.data.options()).await;
    let builder = match question_info {
        Ok(question_info) => {
            problem_view(None, Some(question_info)) 
        }
        Err(why) => {
            tracing::error!("problem error encountered: {}", why);
            problem_view(None, None)
        }
    };

    if let Err(why) = command.create_response(&ctx.http, builder).await {
        tracing::error!("Cannot respond to slash command: {why}");
    }
    
    Ok(())
}

pub async fn run(options: &[ResolvedOption<'_>]) -> Result<ProblemDescription, Error> {
    let mut problem_id: String = "1".to_string();
    let mut use_id_num = false;
    for opt in options {
        match opt.name {
            "identifier" => {
                if let ResolvedValue::String(problem_id_str) = opt.value {
                    problem_id = problem_id_str.to_string();
                }
            }
            "id" => {
                use_id_num = true;
            },
            _ => ()
        }
    }

    if use_id_num {
        problem_id = get_problem_slug_by_id(problem_id.parse::<i32>().unwrap()-1).await?;
    }

    let question_info = get_problem_description(problem_id).await?;
    
    Ok(question_info)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("problem")
        .description("Get information on a leetcode problem")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String, 
                "identifier", 
                "The slug or id of the problem on LeetCode"
            ).required(true))
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean, 
                "id", 
                "Use the problem id to find the problem"
        ))
}
