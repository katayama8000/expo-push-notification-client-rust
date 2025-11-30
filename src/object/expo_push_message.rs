use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::error::ValidationError;
use crate::object::interruption_level::InterruptionLevel;
use crate::object::priority::Priority;
use crate::object::rich_content::RichContent;
use crate::object::sound::Sound;

// <https://docs.expo.dev/push-notifications/sending-notifications/#message-request-format>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpoPushMessage {
    to: Vec<String>,
    title: Option<String>,
    body: Option<String>,
    data: Option<Value>,
    ttl: Option<u64>,
    expiration: Option<u64>,
    priority: Option<Priority>,
    subtitle: Option<String>,
    sound: Option<Sound>,
    badge: Option<u64>,
    channel_id: Option<String>,
    category_id: Option<String>,
    mutable_content: Option<bool>,
    rich_content: Option<RichContent>,
    #[serde(rename = "_contentAvailable")]
    _content_available: Option<bool>,
    interruption_level: Option<InterruptionLevel>,
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
    priority: Option<Priority>,
    subtitle: Option<String>,
    sound: Option<Sound>,
    badge: Option<u64>,
    channel_id: Option<String>,
    category_id: Option<String>,
    mutable_content: Option<bool>,
    rich_content: Option<RichContent>,
    _content_available: Option<bool>,
    interruption_level: Option<InterruptionLevel>,
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
            rich_content: None,
            _content_available: None,
            interruption_level: None,
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

    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn subtitle<S>(mut self, subtitle: S) -> Self
    where
        S: Into<String>,
    {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn sound(mut self, sound: Sound) -> Self {
        self.sound = Some(sound);
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

    pub fn rich_content(mut self, rich_content: RichContent) -> Self {
        self.rich_content = Some(rich_content);
        self
    }

    pub fn content_available(mut self, content_available: bool) -> Self {
        self._content_available = Some(content_available);
        self
    }

    pub fn interruption_level(mut self, interruption_level: InterruptionLevel) -> Self {
        self.interruption_level = Some(interruption_level);
        self
    }

    pub fn build(self) -> Result<ExpoPushMessage, ValidationError> {
        if !self.is_valid_expo_push_token() {
            return Err(ValidationError::InvalidToken);
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
            rich_content: self.rich_content,
            _content_available: self._content_available,
            interruption_level: self.interruption_level,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::rich_content::RichContent;
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
        .priority(Priority::High)
        .subtitle("subtitle")
        .sound(Sound::Default)
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
                priority: Some(Priority::High),
                subtitle: Some("subtitle".to_string()),
                sound: Some(Sound::Default),
                badge: Some(1),
                channel_id: Some("channel_id".to_string()),
                category_id: Some("category_id".to_string()),
                mutable_content: Some(true),
                rich_content: None,
                _content_available: Some(true),
                interruption_level: None,
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
    fn test_expo_push_message_builder_with_rich_content() -> Result<(), ValidationError> {
        let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
            .title("Test")
            .body("Test message")
            .rich_content(RichContent::new().image("https://example.com/image.png"))
            .build()?;

        assert_eq!(
            message,
            ExpoPushMessage {
                to: vec!["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()],
                title: Some("Test".to_string()),
                body: Some("Test message".to_string()),
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
                rich_content: Some(RichContent::new().image("https://example.com/image.png")),
                _content_available: None,
                interruption_level: None,
            }
        );

        let serialized =
            serde_json::to_value(&message).map_err(|_| ValidationError::InvalidData)?;
        let expected_json = json!({
            "to": ["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"],
            "title": "Test",
            "body": "Test message",
            "richContent": {
                "image": "https://example.com/image.png"
            }
        });

        assert_eq!(serialized, expected_json);
        Ok(())
    }

    #[test]
    fn test_expo_push_message_builder_with_empty_rich_content() -> Result<(), ValidationError> {
        let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
            .title("Test")
            .body("Test message")
            .rich_content(RichContent::new())
            .build()?;

        assert_eq!(
            message,
            ExpoPushMessage {
                to: vec!["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()],
                title: Some("Test".to_string()),
                body: Some("Test message".to_string()),
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
                rich_content: Some(RichContent::new()),
                _content_available: None,
                interruption_level: None,
            }
        );

        let serialized =
            serde_json::to_value(&message).map_err(|_| ValidationError::InvalidData)?;
        let expected_json = json!({
            "to": ["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"],
            "title": "Test",
            "body": "Test message",
            "richContent": {}
        });

        assert_eq!(serialized, expected_json);
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
    fn test_expo_push_message_builder_with_interruption_level() -> Result<(), ValidationError> {
        let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
            .title("Test")
            .body("Test message")
            .interruption_level(InterruptionLevel::TimeSensitive)
            .build()?;

        assert_eq!(
            message,
            ExpoPushMessage {
                to: vec!["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()],
                title: Some("Test".to_string()),
                body: Some("Test message".to_string()),
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
                rich_content: None,
                _content_available: None,
                interruption_level: Some(InterruptionLevel::TimeSensitive),
            }
        );

        let serialized =
            serde_json::to_value(&message).map_err(|_| ValidationError::InvalidData)?;
        let expected_json = json!({
            "to": ["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"],
            "title": "Test",
            "body": "Test message",
            "interruptionLevel": "time-sensitive"
        });

        assert_eq!(serialized, expected_json);
        Ok(())
    }

    #[test]
    fn test_expo_push_message_builder_with_all_priorities() -> Result<(), ValidationError> {
        for (priority, expected_str) in [
            (Priority::Default, "default"),
            (Priority::Normal, "normal"),
            (Priority::High, "high"),
        ] {
            let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
                .priority(priority)
                .build()?;

            let serialized =
                serde_json::to_value(&message).map_err(|_| ValidationError::InvalidData)?;
            assert_eq!(serialized["priority"], expected_str);
        }
        Ok(())
    }

    #[test]
    fn test_expo_push_message_builder_with_all_interruption_levels() -> Result<(), ValidationError>
    {
        for (level, expected_str) in [
            (InterruptionLevel::Active, "active"),
            (InterruptionLevel::Critical, "critical"),
            (InterruptionLevel::Passive, "passive"),
            (InterruptionLevel::TimeSensitive, "time-sensitive"),
        ] {
            let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
                .interruption_level(level)
                .build()?;

            let serialized =
                serde_json::to_value(&message).map_err(|_| ValidationError::InvalidData)?;
            assert_eq!(serialized["interruptionLevel"], expected_str);
        }
        Ok(())
    }

    #[test]
    fn test_expo_push_message_builder_with_sound() -> Result<(), ValidationError> {
        let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
            .title("Test")
            .sound(Sound::Default)
            .build()?;

        let serialized =
            serde_json::to_value(&message).map_err(|_| ValidationError::InvalidData)?;
        assert_eq!(serialized["sound"], "default");
        Ok(())
    }

    #[test]
    fn test_expo_push_message_builder_with_custom_sound() -> Result<(), ValidationError> {
        let message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
            .title("Test")
            .sound(Sound::Custom("bells.wav".to_string()))
            .build()?;

        let serialized =
            serde_json::to_value(&message).map_err(|_| ValidationError::InvalidData)?;
        assert_eq!(serialized["sound"], "bells.wav");
        Ok(())
    }
}
