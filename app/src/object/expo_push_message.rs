use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpoPushMessage {
    pub to: Vec<String>,
    pub title: String,
    pub body: String,
    pub data: Option<HashMap<String, Value>>,
    pub ttl: Option<u64>,
    pub expiration: Option<u64>,
    pub priority: Option<String>,
    pub subtitle: Option<String>,
    pub sound: Option<String>,
    pub badge: Option<u64>,
    pub channel_id: Option<String>,
    pub category_id: Option<String>,
    pub mutable_content: Option<bool>,
}

impl ExpoPushMessage {
    pub fn new(to: Vec<String>, title: String, body: String) -> Self {
        ExpoPushMessage {
            to,
            title,
            body,
            data: None,
            ttl: None,
            expiration: None,
            priority: None,
            subtitle: None,
            sound: None,
            badge: None,
            channel_id: None,
            category_id: None,
            mutable_content: None,
        }
    }

    pub fn data(mut self, data: HashMap<String, Value>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn ttl(mut self, ttl: u64) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn expiration(mut self, expiration: u64) -> Self {
        self.expiration = Some(expiration);
        self
    }

    pub fn priority(mut self, priority: String) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn subtitle(mut self, subtitle: String) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn sound(mut self, sound: String) -> Self {
        self.sound = Some(sound);
        self
    }

    pub fn badge(mut self, badge: u64) -> Self {
        self.badge = Some(badge);
        self
    }

    pub fn channel_id(mut self, channel_id: String) -> Self {
        self.channel_id = Some(channel_id);
        self
    }

    pub fn category_id(mut self, category_id: String) -> Self {
        self.category_id = Some(category_id);
        self
    }

    pub fn mutable_content(mut self, mutable_content: bool) -> Self {
        self.mutable_content = Some(mutable_content);
        self
    }
}
