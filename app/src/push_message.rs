use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;

pub async fn push_message(
    expo_push_token: &str,
    title: &str,
    body: &str,
) -> Result<String, String> {
    let url = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    if !expo_push_token.starts_with("ExponentPushToken[") {
        let error_message = format!("Invalid expo push token: {}", expo_push_token);
        return Err(error_message);
    }

    if title.is_empty() {
        let error_message = format!("Title is empty");
        return Err(error_message);
    }

    if body.is_empty() {
        let error_message = format!("Body is empty");
        return Err(error_message);
    }

    let payload = json!({
        "to": expo_push_token,
        "title": title,
        "body": body,
    });

    match client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let body = response
                    .text()
                    .await
                    .expect("Failed to parse response body");
                Ok(body)
            } else {
                let error_message = format!(
                    "Request failed with status code {}: {}",
                    response.status(),
                    response
                        .text()
                        .await
                        .expect("Failed to parse response body")
                );
                Err(error_message)
            }
        }
        Err(err) => {
            let error_message = format!("Failed to send request: {:?}", err);
            Err(error_message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[tokio::test]
    async fn invalid_expo_push_token() {
        let result = push_message("ExponentPushTokenxxxxxxxxxxxxxxxxxxxxxx", "Hello", "World");
        assert_eq!(
            result.await.unwrap_err(),
            "Invalid expo push token: ExponentPushTokenxxxxxxxxxxxxxxxxxxxxxx"
        );
    }

    #[tokio::test]
    async fn empty_title() {
        let result = push_message("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]", "", "World");
        assert_eq!(result.await.unwrap_err(), "Title is empty");
    }

    #[tokio::test]
    async fn empty_body() {
        let result = push_message("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]", "Hello", "");
        assert_eq!(result.await.unwrap_err(), "Body is empty");
    }

    #[tokio::test]
    async fn valid() {
        let mut server = mockito::Server::new();
        server
            .mock("POST", "https://exp.host/--/api/v2/push/send")
            .with_status(200)
            .with_header("content-type", "application/json")
            .match_body(mockito::Matcher::JsonString(
                r#"{"to":"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]","title":"Hello","body":"World"}"#
                    .to_string(),
            )).create();
        let result = push_message(
            "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]",
            "Hello",
            "World",
        );
        assert_eq!(result.await.is_ok(), true);
    }
}
