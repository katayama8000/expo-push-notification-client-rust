use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpoPushMessage {
    to: Vec<String>,
    title: String,
    body: String,
    data: Option<HashMap<String, Vec<String>>>,
    ttl: Option<u64>,
    expiration: Option<u64>,
    priority: Option<String>,
    subtitle: Option<String>,
    sound: Option<String>,
    badge: Option<u64>,
    channel_id: Option<String>,
    category_id: Option<String>,
    mutable_content: Option<bool>,
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

    pub fn data(mut self, data: HashMap<String, Vec<String>>) -> Self {
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

    pub fn is_valid_expo_push_token(&self) -> bool {
        for token in self.to.clone() {
            if !token.starts_with("ExponentPushToken[") {
                return false;
            }
        }
        true
    }

    pub fn is_valid_priority(&self) -> bool {
        if self.priority.is_some() {
            let priority = self.priority.as_ref().unwrap();
            if priority != "default" && priority != "normal" && priority != "high" {
                return false;
            }
        }
        true
    }

    pub fn is_valid_sound(&self) -> bool {
        if self.sound.is_some() {
            let sound = self.sound.as_ref().unwrap();
            if sound != "default" {
                return false;
            }
        }
        true
    }
}
