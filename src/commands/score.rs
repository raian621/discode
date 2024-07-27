use serenity::all::{Color, CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponseFollowup};
use sqlx::PgPool;

use crate::{db::models::LeetCodeConnection, leetcode::query::{completed_questions_count::{get_completed_questions_count, CompletedQuestionsCount}, error::QueryError}};

pub async fn exec(ctx: Context, command: CommandInteraction, pool: &PgPool) -> Result<(), QueryError> {
    tracing::info!("`problem` command executed by `{} ({})`", command.user.name, command.user.id);

    command.defer(&ctx.http).await.unwrap();
    let response =  match run(command.user.id.into(), pool).await? {
        Some(count) => score_response(count),
        None => unlinked_response()
    };

    if let Err(why) = command.create_followup(&ctx.http, response).await {
        tracing::error!("Cannot respond to slash command: {why}");
    }

    Ok(())
}

pub async fn run(discord_id: i64, pool: &PgPool) -> Result<Option<CompletedQuestionsCount>, QueryError> {
    let connection = match LeetCodeConnection::find_with_discord_id(pool, discord_id).await {
        Ok(connection) => Some(connection),
        Err(sqlx::Error::RowNotFound) => None,
        Err(why) => {
            tracing::info!("unexpected error occurred in `problem` execution {}", why);
            return Err(QueryError::Sqlx(why));
        }
    };

    if let Some(connection) = connection {
        let client = reqwest::Client::new();
        Ok(Some(get_completed_questions_count(&client, connection.leetcode_username).await?))
    } else {
        Ok(None)
    }
}

fn score_response(count: CompletedQuestionsCount) -> CreateInteractionResponseFollowup {
    CreateInteractionResponseFollowup::new()
        .add_embed(
            CreateEmbed::new()
                .description(score_markdown(count))
                .color(Color::new(0xffa116))
        )
}

fn unlinked_response() -> CreateInteractionResponseFollowup {
    CreateInteractionResponseFollowup::new()
        .add_embed(
            CreateEmbed::new()
                .description(
                    "No LeetCode account connected, please link your account using the `/connect` command"
                )
                .color(Color::RED)
        )
}

fn score_markdown(count: CompletedQuestionsCount) -> String {
    format!(
        concat!(
            "**Points**: {}\n",
            "## Completed Problems\n",
            "**Easy**: {}\n",
            "**Medium**: {}\n",
            "**Hard**: {}",
        ),
        count.score(),
        count.easy,
        count.medium,
        count.hard,
    )
}

pub fn register() -> CreateCommand {
    CreateCommand::new("score")
        .description("Fetch LeetCode stats from your connected account")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::leetcode::query::completed_questions_count::CompletedQuestionsCount;

    #[test]
    fn test_score_markdown() {
        assert_eq!(
r#"**Points**: 31
## Completed Problems
**Easy**: 2
**Medium**: 3
**Hard**: 4"#,
            score_markdown(CompletedQuestionsCount{
                easy: 2,
                medium: 3,
                hard: 4
            })
        )
    }
}