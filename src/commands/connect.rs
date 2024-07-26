use reqwest::Error;
use serenity::all::{Color, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedValue};
use sqlx::PgPool;

use crate::{db::models::{get_connection_token_by_id, LeetCodeConnection}, leetcode::query::get_user_skills};

enum Status {
    Connected,
    Authenticate(String),
}

pub async fn exec(ctx: Context, command: CommandInteraction, pool: &PgPool) -> Result<(), String> {
    tracing::info!("`connect` command executed by `{} ({})`", command.user.name, command.user.id);
    let leetcode_username = match command.data.options()[0].value {
        ResolvedValue::String(username) => username.to_string(),
        _ => return Err("`username` argument not provided".to_string())
    };

    let builder = match run(command.user.id.into(), &leetcode_username, pool).await {
        Ok(Status::Connected) => connected_view(leetcode_username),
        Ok(Status::Authenticate(token)) => authenticate_view(token),
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

async fn run(discord_id: i64, leetcode_username: &String, pool: &PgPool) -> Result<Status, Error> {
    let connection_token = get_connection_token_by_id(pool, discord_id).await;
    let token: String = format!("discode-{}", connection_token.token);
    let client = reqwest::Client::new();
    let skills = get_user_skills(&client, leetcode_username).await.unwrap();

    let mut authenticated = false;

    for skill in skills {
        if skill == token {
            authenticated = true;
            break;
        }
    }

    match authenticated {
        true => {
            connection_token.delete(pool).await.unwrap();
            LeetCodeConnection {
                discord_id,
                leetcode_username: leetcode_username.clone()
            }.insert(pool).await.unwrap();
            Ok(Status::Connected)
        },
        false => Ok(Status::Authenticate(token))
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

fn authenticate_view(token: String) -> CreateInteractionResponse {
    let embed = CreateEmbed::new()
        .title("Verify Account")
        .description(format!(r#"Add the following token to the skills section on
your LeetCode profile and then re-run the `/connect` command:

```
{}
```

After your account is verified, feel free to delete the skill entry used for
verification in your LeetCode profile's skills section."#,
            token
        ))
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