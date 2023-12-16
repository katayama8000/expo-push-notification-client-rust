use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;

pub struct Body {
    expo_push_token: String,
    title: String,
    body: String,
}

pub async fn push_message(body: Body) -> Result<String, String> {
    let url = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    if !body.expo_push_token.starts_with("ExponentPushToken[") {
        let error_message = format!("Invalid expo push token: {}", body.expo_push_token);
        return Err(error_message);
    }

    if body.title.is_empty() {
        let error_message = format!("Title is empty");
        return Err(error_message);
    }

    if body.body.is_empty() {
        let error_message = format!("Body is empty");
        return Err(error_message);
    }

    let payload = json!({
        "to": body.expo_push_token,
        "title": body.title,
        "body": body.body,
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
        let body = Body {
            expo_push_token: "invalid".to_string(),
            title: "Hello".to_string(),
            body: "World".to_string(),
        };

        let result = push_message(body);
        assert_eq!(
            result.await.unwrap_err(),
            "Invalid expo push token: invalid"
        );
    }

    #[tokio::test]
    async fn empty_title() {
        let body = Body {
            expo_push_token: "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string(),
            title: "".to_string(),
            body: "World".to_string(),
        };
        let result = push_message(body);
        assert_eq!(result.await.unwrap_err(), "Title is empty");
    }

    #[tokio::test]
    async fn empty_body() {
        let body = Body {
            expo_push_token: "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string(),
            title: "Hello".to_string(),
            body: "".to_string(),
        };
        let result = push_message(body);
        assert_eq!(result.await.unwrap_err(), "Body is empty");
    }
    #[tokio::test]
    async fn valid() {
        let body = Body {
            expo_push_token: "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string(),
            title: "Hello".to_string(),
            body: "World".to_string(),
        };
        let mut server = mockito::Server::new();
        server
            .mock("POST", "https://exp.host/--/api/v2/push/send")
            .with_status(200)
            .with_header("content-type", "application/json")
            .match_body(mockito::Matcher::JsonString(
                r#"{"to":"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]","title":"Hello","body":"World"}"#
                    .to_string(),
            )).create();
        let result = push_message(body).await;
        assert_eq!(result.is_ok(), true);
    }
}
