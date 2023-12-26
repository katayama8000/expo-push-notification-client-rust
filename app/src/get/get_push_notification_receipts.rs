use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::Value;

use crate::error::CustomError;
use crate::object::expo_push_error_recept::ExpoPushErrorReceipt;
use crate::object::expo_push_receipt::ExpoPushReceipt;
use crate::object::expo_push_success_recept::ExpoPushSuccessReceipt;
use crate::object::{details::Details, expo_push_receipt_id::ExpoPushReceiptId};

#[derive(Debug, Deserialize, PartialEq)]
struct PushResult {
    data: HashMap<String, PushResultItem>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PushResultItem {
    status: String,
    message: Option<String>,
    details: Option<Value>,
}

pub(crate) async fn get_push_notification_receipts(
    client: reqwest::Client,
    push_ids: ExpoPushReceiptId,
    access_token: Option<String>,
) -> Result<Vec<ExpoPushReceipt>, CustomError> {
    const URL: &str = "https://exp.host/--/api/v2/push/getReceipts";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(token) = access_token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }

    match client
        .post(URL)
        .headers(headers)
        .json(&push_ids)
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
                            ExpoPushSuccessReceipt {
                                status: item.status,
                            },
                        );
                        receipts.push(ExpoPushReceipt::Success(map));
                    } else if item.status == "error" {
                        receipts.push(ExpoPushReceipt::Error(vec![ExpoPushErrorReceipt {
                            status: item.status,
                            message: item.message.unwrap_or_default(),
                            details: item
                                .details
                                .clone()
                                .map(|v| serde_json::from_value::<Details>(v).unwrap()),
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
    #[tokio::test]
    #[ignore]
    async fn test_get_receipts() {
        todo!("test")
    }
}
