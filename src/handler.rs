use std::sync::LazyLock;

use regex::Regex;
use serenity::all::{AutoArchiveDuration, CreateThread, Message, Ready};
use serenity::{async_trait, prelude::*};

use crate::config::{ChannelConfig, Config};

const NUM_CHARS_IN_THREAD_NAME: usize = 100;

pub struct Handler {
    config: Config,
}

impl Handler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

static URL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https?://\S+").expect("Failed to create URL regex"));

fn make_thread_name(msg: &Message) -> String {
    // Replace URLs with empty strings
    let name_with_no_urls = URL_REGEX.replace_all(&msg.content, "").trim().to_string();
    if name_with_no_urls.is_empty() {
        return "Discussion".to_string();
    }
    // Replace URLs with "[url]"
    let mut name = URL_REGEX.replace_all(&msg.content, "[url]").to_string();
    // Remove any newlines from the name
    name = name.replace('\n', " ");
    // Cut the name down to the maximum length
    name.chars().take(NUM_CHARS_IN_THREAD_NAME).collect()
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let Some(ChannelConfig {
            reaction,
            media_only,
            auto_thread,
            ..
        }) = self.config.get_channel_config(msg.channel_id)
        else {
            // If the channel is not configured, do nothing
            return;
        };

        // Try to react to the message
        if let Some(reaction) = reaction {
            if let Err(why) = msg.react(&ctx.http, reaction).await {
                println!("Error reacting to message: {why:?}");
            }
        }

        // Try to delete the message if it is a media-only channel
        // and the message does not contain any attachments or embeds
        if media_only.unwrap_or(false) {
            // Sleep for a few seconds to wait for attachments and embeds to be added
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            // Fetch the message again to fetch attachments and embeds
            let msg = match msg.channel_id.message(&ctx.http, msg.id).await {
                Ok(msg) => msg,
                Err(why) => {
                    println!("Error fetching message: {why:?}");
                    return;
                }
            };
            if msg.attachments.is_empty() && msg.embeds.is_empty() {
                if let Err(why) = msg.delete(&ctx.http).await {
                    println!("Error deleting message: {why:?}");
                }
                // If the message is deleted, we don't need to do anything else
                return;
            }
        }

        // Try to create a thread for the message
        if auto_thread.unwrap_or(false) {
            let name = make_thread_name(&msg);
            if let Err(why) = msg
                .channel_id
                .create_thread_from_message(
                    &ctx.http,
                    msg.id,
                    CreateThread::new(name).auto_archive_duration(AutoArchiveDuration::OneHour),
                )
                .await
            {
                println!("Error creating thread: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
