use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::Value;

use crate::error::CustomError;
use crate::object::{
    Details, ExpoPushErrorReceipt, ExpoPushReceipt, ExpoPushSuccessReceipt,
    GetPushNotificationReceiptsRequest,
};
use crate::ExpoPushReceiptId;

#[derive(Debug, Deserialize, PartialEq)]
struct PushResult {
    data: HashMap<ExpoPushReceiptId, PushResultItem>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PushResultItem {
    status: String,
    message: Option<String>,
    details: Option<Value>,
}

pub(crate) async fn get_push_notification_receipts(
    base_url: &str,
    client: &reqwest::Client,
    push_ids: GetPushNotificationReceiptsRequest,
    access_token: Option<&str>,
) -> Result<HashMap<ExpoPushReceiptId, ExpoPushReceipt>, CustomError> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(token) = access_token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }

    match client
        .post(format!("{}/--/api/v2/push/getReceipts", base_url))
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

                let mut receipts = HashMap::new();
                for (id, item) in result.data {
                    if item.status == "ok" {
                        receipts
                            .insert(id.clone(), ExpoPushReceipt::Success(ExpoPushSuccessReceipt));
                    } else if item.status == "error" {
                        receipts.insert(
                            id.clone(),
                            ExpoPushReceipt::Error(ExpoPushErrorReceipt {
                                message: item.message.unwrap_or_default(),
                                details: item
                                    .details
                                    .clone()
                                    .map(|v| serde_json::from_value::<Details>(v).unwrap()),
                            }),
                        );
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
