use serenity::prelude::*;

mod config;
mod handler;

use handler::Handler;

#[tokio::main]
async fn main() {
    let config = config::load();

    let intents = GatewayIntents::GUILD_MESSAGES;
    let handler = Handler::new(config.clone());

    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
