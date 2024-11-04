use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::error::ValidationError;

// <https://docs.expo.dev/push-notifications/sending-notifications/#message-request-format>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExpoPushMessage {
    to: Vec<String>,
    title: Option<String>,
    body: Option<String>,
    data: Option<Value>,
    ttl: Option<u64>,
    expiration: Option<u64>,
    priority: Option<String>,
    subtitle: Option<String>,
    sound: Option<String>,
    badge: Option<u64>,
    channel_id: Option<String>,
    category_id: Option<String>,
    mutable_content: Option<bool>,
    #[serde(rename = "_contentAvailable")]
    _content_available: Option<bool>,
}

impl ExpoPushMessage {
    pub fn builder<S, I>(to: I) -> ExpoPushMessageBuilder
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        ExpoPushMessageBuilder::new(to.into_iter().map(|s| s.into()).collect::<Vec<String>>())
    }
}

#[derive(Debug)]
pub struct ExpoPushMessageBuilder {
    to: Vec<String>,
    title: Option<String>,
    body: Option<String>,
    data: Option<Value>,
    ttl: Option<u64>,
    expiration: Option<u64>,
    priority: Option<String>,
    subtitle: Option<String>,
    sound: Option<String>,
    badge: Option<u64>,
    channel_id: Option<String>,
    category_id: Option<String>,
    mutable_content: Option<bool>,
    _content_available: Option<bool>,
}

impl ExpoPushMessageBuilder {
    pub(crate) fn new(to: Vec<String>) -> Self {
        ExpoPushMessageBuilder {
            to,
            title: None,
            body: None,
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
            _content_available: None,
        }
    }

    pub fn body<S>(mut self, body: S) -> Self
    where
        S: Into<String>,
    {
        self.body = Some(body.into());
        self
    }

    pub fn data<T>(mut self, data: &T) -> Result<Self, ValidationError>
    where
        T: Serialize,
    {
        match serde_json::to_value(data) {
            Ok(value) => {
                self.data = Some(value);
                Ok(self)
            }
            Err(_) => Err(ValidationError::InvalidData),
        }
    }

    pub fn ttl(mut self, ttl: u64) -> Self {
        self.ttl = Some(ttl);
        self
    }

    pub fn expiration(mut self, expiration: u64) -> Self {
        self.expiration = Some(expiration);
        self
    }

    pub fn priority<S>(mut self, priority: S) -> Self
    where
        S: Into<String>,
    {
        self.priority = Some(priority.into());
        self
    }

    pub fn subtitle<S>(mut self, subtitle: S) -> Self
    where
        S: Into<String>,
    {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn sound<S>(mut self, sound: S) -> Self
    where
        S: Into<String>,
    {
        self.sound = Some(sound.into());
        self
    }

    pub fn badge(mut self, badge: u64) -> Self {
        self.badge = Some(badge);
        self
    }

    pub fn channel_id<S>(mut self, channel_id: S) -> Self
    where
        S: Into<String>,
    {
        self.channel_id = Some(channel_id.into());
        self
    }

    pub fn category_id<S>(mut self, category_id: S) -> Self
    where
        S: Into<String>,
    {
        self.category_id = Some(category_id.into());
        self
    }

    pub fn mutable_content(mut self, mutable_content: bool) -> Self {
        self.mutable_content = Some(mutable_content);
        self
    }

    // for IOS only
    // When this is set to true, the notification will cause the iOS app to start in the background to run a background task.
    // <https://docs.expo.dev/push-notifications/sending-notifications/#message-request-format>
    pub fn content_available(mut self, content_available: bool) -> Self {
        self._content_available = Some(content_available);
        self
    }

    pub fn build(self) -> Result<ExpoPushMessage, ValidationError> {
        if !self.is_valid_expo_push_token() {
            return Err(ValidationError::InvalidToken);
        }

        if !self.is_valid_priority() {
            return Err(ValidationError::InvalidPriority);
        }

        if !self.is_valid_sound() {
            return Err(ValidationError::InvalidSound);
        }

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
            _content_available: self._content_available,
        };

        Ok(message)
    }

    pub fn title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    fn is_valid_expo_push_token(&self) -> bool {
        self.to.iter().all(|token| {
            ((token.starts_with("ExponentPushToken[") || token.starts_with("ExpoPushToken["))
                && token.ends_with(']'))
                || regex::Regex::new(r"^[a-z\d]{8}-[a-z\d]{4}-[a-z\d]{4}-[a-z\d]{4}-[a-z\d]{12}$")
                    .expect("regex is valid")
                    .is_match(token)
        })
    }

    fn is_valid_priority(&self) -> bool {
        self.priority
            .as_ref()
            .map(|p| p == "default" || p == "normal" || p == "high")
            .unwrap_or(true)
    }

    fn is_valid_sound(&self) -> bool {
        self.sound.as_ref().map(|s| s == "default").unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_expo_push_message_builder() -> Result<(), ValidationError> {
        #[derive(Serialize)]
        struct Data {
            data: String,
        }
        let message = ExpoPushMessage::builder([
            "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]",
            "ExpoPushToken[xxxxxxxxxxxxxxxxxxxxxx]",
            "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
        ])
        .body("body")
        .data(&Data {
            data: "data".to_string(),
        })?
        .ttl(100)
        .expiration(100)
        .priority("high")
        .subtitle("subtitle")
        .sound("default")
        .badge(1)
        .channel_id("channel_id")
        .category_id("category_id")
        .mutable_content(true)
        .title("title")
        .content_available(true)
        .build()?;

        assert_eq!(
            message,
            ExpoPushMessage {
                to: vec![
                    "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string(),
                    "ExpoPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string(),
                    "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string(),
                ],
                title: Some("title".to_string()),
                body: Some("body".to_string()),
                data: Some(json!({ "data": "data" })),
                ttl: Some(100),
                expiration: Some(100),
                priority: Some("high".to_string()),
                subtitle: Some("subtitle".to_string()),
                sound: Some("default".to_string()),
                badge: Some(1),
                channel_id: Some("channel_id".to_string()),
                category_id: Some("category_id".to_string()),
                mutable_content: Some(true),
                _content_available: Some(true),
            }
        );

        let expected_json = json!({
            "to": [
                "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]",
                "ExpoPushToken[xxxxxxxxxxxxxxxxxxxxxx]",
                "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
            ],
            "title": "title",
            "body": "body",
            "data": {
                "data": "data"
            },
            "ttl": 100,
            "expiration": 100,
            "priority": "high",
            "subtitle": "subtitle",
            "sound": "default",
            "badge": 1,
            "channelId": "channel_id",
            "categoryId": "category_id",
            "mutableContent": true,
            "_contentAvailable": true
        });

        let serialized_message =
            serde_json::to_value(&message).map_err(|_| ValidationError::InvalidData)?;
        let expected_message =
            serde_json::to_value(&expected_json).map_err(|_| ValidationError::InvalidData)?;
        assert_eq!(serialized_message, expected_message);
        Ok(())
    }

    #[test]
    fn test_expo_push_message_builder_invalid_token() {
        let message = ExpoPushMessage::builder([
            "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]",
            "invalid_token",
        ])
        .build();

        assert_eq!(message, Err(ValidationError::InvalidToken));
    }

    #[test]
    fn test_expo_push_message_builder_invalid_priority() {
        let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
            .priority("invalid_priority")
            .build();

        assert_eq!(message, Err(ValidationError::InvalidPriority));
    }

    #[test]
    fn expo_push_message_builder_invalid_sound() {
        let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
            .sound("invalid_sound")
            .build();

        assert_eq!(message, Err(ValidationError::InvalidSound));
    }
}
