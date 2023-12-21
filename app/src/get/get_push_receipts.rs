use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;

use crate::error::CustomError;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct PushReceiptResponse {
    pub data: serde_json::Value,
    pub errors: Option<Vec<serde_json::Value>>,
}
pub async fn get_push_receipts(ids: &[&str]) -> Result<PushReceiptResponse, CustomError> {
    const URL: &str = "https://exp.host/--/api/v2/push/getReceipts";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    for id in ids {
        if id.is_empty() {
            return Err(CustomError::InvalidArgument("id is empty".to_string()));
        }
    }

    let payload = json!({
        "ids": ids,
    });

    match client
        .post(URL)
        .headers(headers)
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let result = response
                    .json::<PushReceiptResponse>()
                    .await
                    .expect("Failed to parse response body");
                Ok(result)
            } else {
                Err(CustomError::ServerErr(format!(
                    "Request failed: {}",
                    response.status()
                )))
            }
        }
        Err(err) => Err(CustomError::ServerErr(format!("Request failed: {}", err))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_get_receipts() {
        todo!("need to mock the response")
    }

    #[tokio::test]
    async fn test_get_receipts_empty_id() {
        let ids = [""];
        let result = get_push_receipts(&ids).await;
        assert_eq!(
            result,
            Err(CustomError::InvalidArgument("id is empty".to_string()))
        );
    }
}
