use crate::leetcode::{models::ProblemDescription, query::{get_problem_description, get_problem_slug_by_id}, views::problem_view};

use reqwest::Error;
use serenity::all::{CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue};

#[derive(PartialEq, Debug)]
enum IdType {
    Unknown,
    IdNumber,
    Slug
}

impl IdType {
    fn new(value: ResolvedValue) -> Self {
        if let ResolvedValue::String(value) = value {
            match value {
                "id" => Self::IdNumber,
                "slug" => Self::Slug,
                _ => Self::Unknown
            }
        } else {
            Self::Unknown
        }
    }
}

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
    let (mut problem_id, id_type) = extract_options(
        options.iter().map(|opt| (opt.name, opt.value.clone())).collect()
    );
    
    let client = reqwest::Client::new();
    let question_info = match id_type {
        IdType::IdNumber => {
            let id_num = problem_id.parse::<i32>().unwrap_or(0);
            problem_id = get_problem_slug_by_id(id_num).await?;
            get_problem_description(&client, problem_id).await?
        },
        IdType::Slug|IdType::Unknown => get_problem_description(&client, problem_id).await?,
    };
    
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
 
fn extract_options(options: Vec<(&str, ResolvedValue)>) -> (String, IdType) {
    let mut problem_id: String = "1".to_string();
    let mut id_type = IdType::Unknown;

    for (name, value) in options {
        match name {
            "identifier" => {
                // identifier is a required option, so this block will always execute
                if let ResolvedValue::String(problem_id_str) = value {
                    problem_id = problem_id_str.to_string();
                }
            },
            "type" => {
                id_type = IdType::new(value.clone())
            },
            _ => ()
        }
    }

    (problem_id, id_type)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_id_type_new() {
        struct TestCase<'a> {
            value: ResolvedValue<'a>,
            want: IdType
        }

        let test_cases = [
            TestCase {
                value: ResolvedValue::String("id"),
                want: IdType::IdNumber
            },
            TestCase {
                value: ResolvedValue::String("slug"),
                want: IdType::Slug
            },
            TestCase {
                value: ResolvedValue::String(""),
                want: IdType::Unknown
            },
            TestCase {
                value: ResolvedValue::String("unknown-lol"),
                want: IdType::Unknown
            },
            TestCase {
                value: ResolvedValue::Number(12.0),
                want: IdType::Unknown
            }
        ];

        for tc in test_cases {
            assert_eq!(IdType::new(tc.value.clone()), tc.want);
        }
    }

    #[test]
    fn test_extract_options() {
        #[non_exhaustive]
        struct TestCase<'a> {
            options: Vec<(&'a str, ResolvedValue<'a>)>,
            want_problem_id: String,
            want_id_type: IdType
        }
        
        let test_cases = [
            TestCase {
                options: vec![
                    ("identifier", ResolvedValue::String("jump-game"))
                ],
                want_problem_id:  "jump-game".to_string(),
                want_id_type: IdType::Unknown,
            },
            TestCase {
                options: vec![
                    ("identifier", ResolvedValue::String("jump-game")),
                    ("type", ResolvedValue::String("slug"))
                ],
                want_problem_id:  "jump-game".to_string(),
                want_id_type: IdType::Slug,
            },
            TestCase {
                options: vec![
                    ("identifier", ResolvedValue::String("55")),
                    ("type", ResolvedValue::String("id"))
                ],
                want_problem_id:  "55".to_string(),
                want_id_type: IdType::IdNumber,
            }
        ];

        for tc in test_cases {
            let (problem_id, id_type) = extract_options(tc.options);
            assert_eq!(problem_id, tc.want_problem_id);
            assert_eq!(id_type, tc.want_id_type);
        }
    }
}
