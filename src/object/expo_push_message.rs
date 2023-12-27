use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use crate::error::ValidationError;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
    pub fn is_valid_expo_push_token(&self) -> bool {
        self.to
            .iter()
            .all(|token| token.starts_with("ExponentPushToken["))
    }

    pub fn is_valid_priority(&self) -> bool {
        self.priority
            .as_ref()
            .map(|p| p == "default" || p == "normal" || p == "high")
            .unwrap_or(true)
    }

    pub fn is_valid_sound(&self) -> bool {
        self.sound.as_ref().map(|s| s == "default").unwrap_or(true)
    }
}

#[derive(Debug)]
pub struct ExpoPushMessageBuilder {
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

impl ExpoPushMessageBuilder {
    pub fn new(to: Vec<String>, title: String, body: String) -> Self {
        ExpoPushMessageBuilder {
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

    pub fn build(self) -> Result<ExpoPushMessage, ValidationError> {
        let message = ExpoPushMessage {
            to: self.to,
            title: self.title,
            body: self.body,
            data: self.data,
            ttl: self.ttl,
            expiration: self.expiration,
            priority: self.priority,
            subtitle: self.subtitle,
            sound: self.sound,
            badge: self.badge,
            channel_id: self.channel_id,
            category_id: self.category_id,
            mutable_content: self.mutable_content,
        };

        if !message.is_valid_expo_push_token() {
            return Err(ValidationError::InvalidToken);
        }

        if !message.is_valid_priority() {
            return Err(ValidationError::InvalidPriority);
        }

        if !message.is_valid_sound() {
            return Err(ValidationError::InvalidSound);
        }

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expo_push_message_builder() -> Result<(), ValidationError> {
        let message = ExpoPushMessageBuilder::new(
            vec!["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()],
            "title".to_string(),
            "body".to_string(),
        )
        .data(
            [("key".to_string(), vec!["value".to_string()])]
                .iter()
                .cloned()
                .collect(),
        )
        .ttl(100)
        .expiration(100)
        .priority("high".to_string())
        .subtitle("subtitle".to_string())
        .sound("default".to_string())
        .badge(1)
        .channel_id("channel_id".to_string())
        .category_id("category_id".to_string())
        .mutable_content(true)
        .build()?;

        assert_eq!(
            message,
            ExpoPushMessage {
                to: vec!["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()],
                title: "title".to_string(),
                body: "body".to_string(),
                data: Some(
                    [("key".to_string(), vec!["value".to_string()])]
                        .iter()
                        .cloned()
                        .collect()
                ),
                ttl: Some(100),
                expiration: Some(100),
                priority: Some("high".to_string()),
                subtitle: Some("subtitle".to_string()),
                sound: Some("default".to_string()),
                badge: Some(1),
                channel_id: Some("channel_id".to_string()),
                category_id: Some("category_id".to_string()),
                mutable_content: Some(true),
            }
        );
        Ok(())
    }

    #[test]
    fn test_expo_push_message_builder_invalid_token() {
        let message = ExpoPushMessageBuilder::new(
            vec![
                "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string(),
                "invalid_token".to_string(),
            ],
            "title".to_string(),
            "body".to_string(),
        )
        .build();

        assert_eq!(message, Err(ValidationError::InvalidToken));
    }

    #[test]
    fn test_expo_push_message_builder_invalid_priority() {
        let message = ExpoPushMessageBuilder::new(
            vec!["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()],
            "title".to_string(),
            "body".to_string(),
        )
        .priority("invalid_priority".to_string())
        .build();

        assert_eq!(message, Err(ValidationError::InvalidPriority));
    }
}
