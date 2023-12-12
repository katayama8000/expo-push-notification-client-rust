use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;

type PushResult = Result<String, String>;

pub async fn push_message(expo_push_token: &str, title: &str, body: &str) -> PushResult {
    let url = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

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
