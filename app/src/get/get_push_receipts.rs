use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;

use crate::error::CustomError;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum PushReceipt {
    Success(HashMap<String, PushSuccessReceipt>),
    Error(Vec<PushErrorReceipt>),
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PushSuccessReceipt {
    pub status: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PushErrorReceipt {
    pub status: String,
    pub message: String,
    pub details: Value,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PushResult {
    pub data: HashMap<String, PushResultItem>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PushResultItem {
    pub status: String,
    pub message: Option<String>,
    pub details: Option<Value>,
}

pub async fn get_push_receipts(ids: Vec<String>) -> Result<Vec<PushReceipt>, CustomError> {
    const URL: &str = "https://exp.host/--/api/v2/push/getReceipts";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    for id in ids.clone() {
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
                let result: PushResult = response.json::<PushResult>().await.map_err(|err| {
                    CustomError::DeserializeErr(format!("Failed to deserialize response: {}", err))
                })?;

                let mut receipts = Vec::new();
                for (id, item) in result.data {
                    if item.status == "ok" {
                        let mut map = HashMap::new();
                        map.insert(
                            id.clone(),
                            PushSuccessReceipt {
                                status: item.status,
                            },
                        );
                        receipts.push(PushReceipt::Success(map));
                    } else if item.status == "error" {
                        receipts.push(PushReceipt::Error(vec![PushErrorReceipt {
                            status: item.status,
                            message: item.message.unwrap_or_default(),
                            details: item.details.unwrap_or_default(),
                        }]));
                    } else {
                        return Err(CustomError::DeserializeErr(format!(
                            "Unknown status: {}",
                            item.status
                        )));
                    }
                }
                Ok(receipts)
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
        let ids = vec!["".to_string()];
        let result = get_push_receipts(ids).await;
        assert_eq!(
            result,
            Err(CustomError::InvalidArgument("id is empty".to_string()))
        );
    }
}
