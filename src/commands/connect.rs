use reqwest::Error;
use serenity::all::{Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedValue};
use sqlx::PgPool;

use crate::{db::models::{get_connection_token_by_id, LeetCodeConnection}, leetcode::query::get_user_skills};

enum Status {
    Connected,
    Validate(String),
}

pub async fn exec(ctx: Context, command: CommandInteraction, pool: &PgPool) -> Result<(), String> {
    tracing::info!("`connect` command executed by `{} ({})`", command.user.name, command.user.id);
    let leetcode_username = match command.data.options()[0].value {
        ResolvedValue::String(username) => username.to_string(),
        _ => return Err("`username` argument not provided".to_string())
    };

    let result = run(pool, command.user.id.into(), &leetcode_username).await;
    let builder = match result {
        Ok(Status::Connected) => connected_view(leetcode_username),
        Ok(Status::Validate(token)) => validate_view(token),
        Err(why) => {
            tracing::error!("{}", why);
            error_view()
        }
    };

    if let Err(why) = command.create_response(&ctx.http, builder).await {
        println!("Cannot respond to slash command: {why}");
    }
    
    Ok(())
}

async fn run(
    pool: &PgPool,
    discord_id: i64,
    leetcode_username: &String,
) -> Result<Status, Error> {
    let connection_token = get_connection_token_by_id(pool, discord_id).await;
    let token: String = format!("discode-{}", connection_token.token);
    let client = reqwest::Client::new();
    let skills = get_user_skills(&client, leetcode_username).await.unwrap();

    let authenticated = validation_token_present(&skills, &token);

    match authenticated {
        true => {
            connection_token.delete(pool).await.unwrap();
            LeetCodeConnection {
                discord_id,
                leetcode_username: leetcode_username.clone()
            }.insert(pool).await.unwrap();
            Ok(Status::Connected)
        },
        false => Ok(Status::Validate(token))
    }
}

fn connected_view(leetcode_username: String) -> CreateInteractionResponse {
    let embed = CreateEmbed::new()
        .description(format!(
            "Successfully connected account [{}](https://leetcode.com/u/{}/)!",
            leetcode_username,
            leetcode_username
        ))
        .color(Color::new(0xffa116));

    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .add_embed(embed)
            .ephemeral(true)
    )
}

fn validate_view(token: String) -> CreateInteractionResponse {
    let embed = CreateEmbed::new()
        .title("Verify Account")
        .description(verify_account_markdown(token))
        .color(Color::new(0xffa116));

    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .add_embed(embed)
            .ephemeral(true)
    )
}

fn error_view() -> CreateInteractionResponse {
    let embed = CreateEmbed::new()
        .title("Error")
        .description("An unexpected error occurred, please try again later")
        .color(Color::new(0xffa116));

    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .add_embed(embed)
            .ephemeral(true)
    )
}

pub fn register() -> CreateCommand {
    CreateCommand::new("connect")
        .description("Connect your LeetCode account to DisCode")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "username",
                "Your LeetCode username"
            )
            .required(true)
        )
}

fn validation_token_present<T: Into<String> + Clone>(skills: &[T], token: &T) -> bool {
    let token: String = token.clone().into();
    for skill in skills.iter() {
        if skill.clone().into() == token {
            return true;
        }
    }

    false
}

fn verify_account_markdown(token: impl Into<String>) -> String {
    format!(
r#"Add the following token to the skills section on
your LeetCode profile and then re-run the `/connect` command:

```
{}
```

After your account is verified, feel free to delete the skill entry used for
verification in your LeetCode profile's skills section."#,
        token.into()
    )
}

#[cfg(test)]
mod tests {
    use crate::commands::connect::{validation_token_present, verify_account_markdown};

    #[test]
    fn test_validation_token_present() {
        struct TestCase<'a> {
            pub skills: Vec<&'a str>,
            pub token: &'a str,
            pub want: bool
        }

        let test_cases = [
            TestCase {
                skills: vec!["C++", "Java", "Python", "discode-abcd9999"],
                token: "discode-abcd9999",
                want: true
            },
            TestCase {
                skills: vec!["C++", "Java", "Python", "discode-wxyz0000"],
                token: "discode-abcd9999",
                want: false
            },
            TestCase {
                skills: vec!["C++", "Java", "Python"],
                token: "discode-abcd9999",
                want: false
            },
        ];

        for tc in test_cases {
            assert_eq!(tc.want, validation_token_present(&tc.skills, &tc.token))
        }
    }

    #[test]
    fn test_verify_account_markdown() {
        assert_eq!(
            verify_account_markdown("discode-abcd9921"),
            r#"Add the following token to the skills section on
your LeetCode profile and then re-run the `/connect` command:

```
discode-abcd9921
```

After your account is verified, feel free to delete the skill entry used for
verification in your LeetCode profile's skills section."#,
        );
    }
}