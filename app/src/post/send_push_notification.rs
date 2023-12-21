use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PushTicket {
    Success(PushSuccessTicket),
    Error(PushErrorTicket),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushSuccessTicket {
    pub status: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PushErrorTicket {
    pub status: String,
    pub message: String,
    pub details: Details,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Details {
    pub error: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Error {
    InvalidArgument(String),
    DeserializeErr(String),
    ServerErr(String),
}

#[derive(Debug, Serialize)]
pub struct PushPayload<'a> {
    to: &'a [&'a str],
    title: &'a str,
    body: &'a str,
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
    details: Option<Details>,
}

pub async fn send_push_notification(
    expo_push_tokens: &[&str],
    title: &str,
    body: &str,
) -> Result<Vec<PushTicket>, Error> {
    const URL: &str = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    for token in expo_push_tokens {
        if !token.starts_with("ExponentPushToken[") {
            return Err(Error::InvalidArgument(format!(
                "Invalid expo push token: {}",
                token
            )));
        }
    }

    if title.is_empty() {
        return Err(Error::InvalidArgument("Title is empty".to_string()));
    }

    if body.is_empty() {
        return Err(Error::InvalidArgument("Body is empty".to_string()));
    }

    let payload = PushPayload {
        to: expo_push_tokens,
        title,
        body,
    };

    match client
        .post(URL)
        .headers(headers)
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response
                    .json::<PushResult>()
                    .await
                    .map_err(|err| {
                        Error::DeserializeErr(format!("Failed to deserialize response: {:?}", err))
                    })?
                    .data
                    .into_iter()
                    .map(|item| {
                        if item.status == "error" {
                            PushTicket::Error(PushErrorTicket {
                                status: item.status,
                                message: item.message.unwrap_or_default(), // Use unwrap_or_default to provide a default value
                                details: item.details.unwrap_or_default(), // Use unwrap_or_default to provide a default value
                            })
                        } else if item.status == "ok" {
                            PushTicket::Success(PushSuccessTicket {
                                status: item.status,
                                id: item.id.unwrap_or_default(), // Use unwrap_or_default to provide a default value
                            })
                        } else {
                            unreachable!("Unknown status: {}", item.status)
                        }
                    })
                    .collect())
            } else {
                Err(Error::ServerErr(format!(
                    "Failed to send request: {:?}",
                    response
                )))
            }
        }
        Err(err) => Err(Error::ServerErr(format!(
            "Failed to send request: {:?}",
            err
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use mockito;

    #[tokio::test]
    async fn test_invalid_expo_push_token() {
        let result = send_push_notification(&["invalid_token"], "Hello", "World").await;
        assert_eq!(
            result.unwrap_err(),
            Error::InvalidArgument("Invalid expo push token: invalid_token".to_string())
        );
    }

    #[tokio::test]
    async fn test_empty_title() {
        let result =
            send_push_notification(&["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"], "", "World")
                .await;
        assert_eq!(
            result.unwrap_err(),
            Error::InvalidArgument("Title is empty".to_string())
        );
    }

    #[tokio::test]
    async fn test_empty_body() {
        let result =
            send_push_notification(&["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"], "Hello", "")
                .await;
        assert_eq!(
            result.unwrap_err(),
            Error::InvalidArgument("Body is empty".to_string())
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
        todo!("test invalid post")
    }
}
