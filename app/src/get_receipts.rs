use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;

pub async fn get_receipts(ids: &[&str]) -> Result<String, String> {
    let url = "https://exp.host/--/api/v2/push/getReceipts";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    for id in ids {
        if id.is_empty() {
            let error_message = format!("id is empty");
            return Err(error_message);
        }
    }

    let payload = json!({
        "ids": ids,
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
            let error_message = format!("Request failed: {}", err);
            Err(error_message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_get_receipts() {
        // TODO: need to mock the response
        todo!("need to mock the response")
    }

    #[tokio::test]
    async fn test_get_receipts_empty_id() {
        let ids = [""];
        let result = get_receipts(&ids).await;
        assert_eq!(result, Err("id is empty".to_string()));
    }
}
