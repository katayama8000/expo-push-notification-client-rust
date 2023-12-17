use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(default)]
    pub data: Vec<PushTicket>,
}

#[derive(Debug, Deserialize)]
pub struct PushTicket {
    pub status: String,
    pub id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
    pub details: Value,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Error {
    InvalidArgument(String),
    ExpoErr(ErrorResponse),
    DeserializeErr(String), // 新しいエラー型を追加
    Others(String),
}

#[derive(Debug, Serialize)]
struct PushPayload<'a> {
    to: &'a [&'a str],
    title: &'a str,
    body: &'a str,
}

pub async fn push_message(
    expo_push_tokens: &[&str],
    title: &str,
    body: &str,
) -> Result<ApiResponse, Error> {
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
                let body = response.json::<ApiResponse>().await.map_err(|err| {
                    Error::DeserializeErr(format!(
                        "Failed to parse response body as ApiResponse: {:?}",
                        err
                    ))
                })?;
                Ok(body)
            } else {
                Err(Error::ExpoErr(
                    response.json::<ErrorResponse>().await.map_err(|err| {
                        Error::DeserializeErr(format!(
                            "Failed to parse response body as ErrorResponse: {:?}",
                            err
                        ))
                    })?,
                ))
            }
        }
        Err(err) => Err(Error::Others(format!("Failed to send request: {:?}", err))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use mockito;

    #[tokio::test]
    async fn test_invalid_expo_push_token() {
        let result = push_message(&["invalid_token"], "Hello", "World").await;
        assert_eq!(
            result.unwrap_err(),
            Error::InvalidArgument("Invalid expo push token: invalid_token".to_string())
        );
    }

    #[tokio::test]
    async fn test_empty_title() {
        let result =
            push_message(&["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"], "", "World").await;
        assert_eq!(
            result.unwrap_err(),
            Error::InvalidArgument("Title is empty".to_string())
        );
    }

    #[tokio::test]
    async fn test_empty_body() {
        let result =
            push_message(&["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"], "Hello", "").await;
        assert_eq!(
            result.unwrap_err(),
            Error::InvalidArgument("Body is empty".to_string())
        );
    }

    #[tokio::test]
    #[ignore]
    async fn test_valid_post() {
        // let mut server = mockito::Server::new();
        // server
        //     .mock("POST", "https://exp.host/--/api/v2/push/send")
        //     .with_status(200)
        //     .with_header("content-type", "application/json")
        //     .match_body(mockito::Matcher::JsonString(
        //         r#"{"to":["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"],"title":"Hello","body":"World"}"#
        //             .to_string(),
        //     )).create();
        // let result = push_message(
        //     &["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"],
        //     "Hello",
        //     "World",
        // )
        // .await;
        // assert!(result.is_err());
        todo!("can not parse response body")
    }

    #[tokio::test]
    #[ignore]
    async fn test_invalid_post() {
        todo!("test invalid post")
    }
}
