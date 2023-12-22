use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    error::CustomError,
    object::{
        details::Details, expo_push_message::ExpoPushMessage,
        expo_push_success_ticket::ExpoPushSuccessTicket,
    },
    object::{expo_push_error_ticket::ExpoPushErrorTicket, expo_push_ticket::ExpoPushTicket},
};

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
