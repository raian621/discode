mod commands;
mod leetcode;

use std::env;
use dotenv::dotenv;
use serenity::all::{GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "ping" => { commands::ping::run(&command.data.options()); },
                "daily" => commands::daily::exec(ctx, command).await.unwrap(),
                "problem" => commands::problem::exec(ctx, command).await.unwrap(),
                _ => { println!("not implemented :("); },
            };

            // if let Some(content) = content {
            //     let data = CreateInteractionResponseMessage::new().content(content);
            //     let builder = CreateInteractionResponse::Message(data);
            //     if let Err(why) = command.create_response(&ctx.http, builder).await {
            //         println!("Cannot respond to slash command: {why}");
            //     }
            // }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID").expect("Expected GUILD_ID environment variable to be set").parse().expect("GUILD_ID must be an integer"),
        );

        guild_id.set_commands(&ctx.http, vec![
            commands::ping::register(),
            commands::daily::register(),
            commands::problem::register()
        ]).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // load environment variables from file
    dotenv().ok();
    // init logger
    tracing_subscriber::fmt::init();
    // Login with a bot token from the environment
    let token: String = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        tracing::error!("Client error: {why:?}");
    }
}