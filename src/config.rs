use serde::Deserialize;
use serenity::all::{EmojiId, ReactionType};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub discord_token: String,
    pub channels: Vec<ChannelConfig>,
}

impl Config {
    pub fn get_channel_config<T>(&self, channel_id: T) -> Option<ChannelConfig>
    where
        T: Into<u64> + Copy,
    {
        self.channels
            .iter()
            .find(|channel| channel.id == channel_id.into())
            .cloned()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChannelConfig {
    pub id: u64,
    pub reaction: Option<ReactionConfig>,
    pub media_only: Option<bool>,
    pub auto_thread: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum ReactionConfig {
    Unicode(String),
    Emoji { id: u64, name: String },
}

impl From<ReactionConfig> for ReactionType {
    fn from(val: ReactionConfig) -> Self {
        match val {
            ReactionConfig::Unicode(s) => ReactionType::Unicode(s),
            ReactionConfig::Emoji { id, name } => ReactionType::Custom {
                animated: false,
                id: EmojiId::new(id),
                name: Some(name),
            },
        }
    }
}

pub fn load() -> Config {
    toml::from_str(&std::fs::read_to_string("config.toml").unwrap()).unwrap()
}
