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
            "type" => {
                if let ResolvedValue::String(choice) = opt.value {
                    match choice {
                        "id" => use_id_num = true,
                        "slug" => (),
                        _ => ()
                    }
                }
            },
            _ => ()
        }
    }

    if use_id_num {
        let id_num = problem_id.parse::<i32>().unwrap_or(0);
        problem_id = get_problem_slug_by_id(id_num).await?;
    }

    let client = reqwest::Client::new();
    let question_info = get_problem_description(&client, problem_id).await?;
    
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
                CommandOptionType::String, 
                "type", 
                "What to use to find the problem (problem id, title slug, etc.)"
            )
            .add_string_choice("id", "id")
            .add_string_choice("slug", "slug")
        )
}
