use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::CustomError, get_push_notification_receipts::Details};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExpoPushTicket {
    Success(ExpoPushSuccessTicket),
    Error(ExpoPushErrorTicket),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpoPushSuccessTicket {
    pub status: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpoPushErrorTicket {
    pub status: String,
    pub message: String,
    pub details: Option<Details>,
}

#[derive(Debug, Deserialize)]
struct PushResult {
    data: Vec<PushResultItem>,
}

#[derive(Debug, Deserialize)]
struct PushResultItem {
    status: String,
    id: Option<String>,
    message: Option<String>,
    details: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpoPushMessage {
    to: Vec<String>,
    title: String,
    body: String,
    data: Option<serde_json::Value>,
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

    pub fn data(mut self, data: Value) -> Self {
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

pub async fn send_push_notifications(
    push_message: ExpoPushMessage,
    access_token: Option<String>,
) -> Result<Vec<ExpoPushTicket>, CustomError> {
    const URL: &str = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(token) = access_token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }

    let client = reqwest::Client::new();

    for token in push_message.to.clone() {
        if !token.starts_with("ExponentPushToken[") {
            return Err(CustomError::InvalidArgument(format!(
                "Invalid expo push token: {}",
                token
            )));
        }
    }

    if push_message.title.is_empty() {
        return Err(CustomError::InvalidArgument("Title is empty".to_string()));
    }

    if push_message.body.is_empty() {
        return Err(CustomError::InvalidArgument("Body is empty".to_string()));
    }

    if push_message.priority.is_some() {
        let priority = push_message.priority.as_ref().unwrap();
        if priority != "default" && priority != "normal" && priority != "high" {
            return Err(CustomError::InvalidArgument(format!(
                "Invalid priority: {}",
                priority
            )));
        }
    }

    if push_message.sound.is_some() {
        let sound = push_message.sound.as_ref().unwrap();
        if sound != "default" {
            return Err(CustomError::InvalidArgument(format!(
                "Invalid sound: {}",
                sound
            )));
        }
    }

    match client
        .post(URL)
        .headers(headers)
        .json(&push_message)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response
                    .json::<PushResult>()
                    .await
                    .map_err(|err| {
                        CustomError::DeserializeErr(format!(
                            "Failed to deserialize response: {:?}",
                            err
                        ))
                    })?
                    .data
                    .into_iter()
                    .map(|item| {
                        if item.status == "error" {
                            ExpoPushTicket::Error(ExpoPushErrorTicket {
                                status: item.status,
                                message: item.message.expect("message is empty"),
                                details: item
                                    .details
                                    .map(|v| serde_json::from_value::<Details>(v).unwrap()),
                            })
                        } else if item.status == "ok" {
                            ExpoPushTicket::Success(ExpoPushSuccessTicket {
                                status: item.status,
                                id: item.id.expect("id is empty"),
                            })
                        } else {
                            unreachable!("Unknown status: {}", item.status)
                        }
                    })
                    .collect())
            } else {
                Err(CustomError::ServerErr(format!(
                    "Failed to send request: {:?}",
                    response
                )))
            }
        }
        Err(err) => Err(CustomError::ServerErr(format!(
            "Failed to send request: {:?}",
            err
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_expo_push_token() {
        let expo_push_message = ExpoPushMessage::new(
            vec![String::from("invalid_token")],
            "Hello".to_string(),
            "World".to_string(),
        );
        let result = send_push_notifications(expo_push_message, None).await;
        assert_eq!(
            result.unwrap_err(),
            CustomError::InvalidArgument("Invalid expo push token: invalid_token".to_string())
        );
    }

    #[tokio::test]
    async fn test_empty_title() {
        let expo_push_message = ExpoPushMessage::new(
            vec![String::from("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]")],
            "".to_string(),
            "World".to_string(),
        );
        let result = send_push_notifications(expo_push_message, None).await;
        assert_eq!(
            result.unwrap_err(),
            CustomError::InvalidArgument("Title is empty".to_string())
        );
    }

    #[tokio::test]
    async fn test_empty_body() {
        let expo_push_message = ExpoPushMessage::new(
            vec![String::from("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]")],
            "Hello".to_string(),
            "".to_string(),
        );
        let result = send_push_notifications(expo_push_message, None).await;
        assert_eq!(
            result.unwrap_err(),
            CustomError::InvalidArgument("Body is empty".to_string())
        );
    }

    #[tokio::test]
    async fn test_invalid_priority() {
        let expo_push_message = ExpoPushMessage::new(
            vec![String::from("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]")],
            "Hello".to_string(),
            "World".to_string(),
        )
        .priority("invalid_priority".to_string());
        let result = send_push_notifications(expo_push_message, None).await;
        assert_eq!(
            result.unwrap_err(),
            CustomError::InvalidArgument("Invalid priority: invalid_priority".to_string())
        );
    }

    #[tokio::test]
    async fn test_invalid_sound() {
        let expo_push_message = ExpoPushMessage::new(
            vec![String::from("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]")],
            "Hello".to_string(),
            "World".to_string(),
        )
        .sound("invalid_sound".to_string());
        let result = send_push_notifications(expo_push_message, None).await;
        assert_eq!(
            result.unwrap_err(),
            CustomError::InvalidArgument("Invalid sound: invalid_sound".to_string())
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_valid_post() {
        todo!("test")
    }

    #[tokio::test]
    #[ignore]
    async fn test_invalid_post() {
        todo!("test")
    }
}
