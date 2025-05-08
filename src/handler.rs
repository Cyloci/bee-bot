use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::config::{ChannelConfig, Config};

pub struct Handler {
    config: Config,
}

impl Handler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let channel_config = self.config.get_channel_config(msg.channel_id);

        // Try to react to the message
        if let Some(ChannelConfig {
            reaction: Some(reaction),
            ..
        }) = channel_config
        {
            if let Err(why) = msg.react(&ctx.http, reaction).await {
                println!("Error reacting to message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
