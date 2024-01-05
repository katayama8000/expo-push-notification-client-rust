use std::collections::HashMap;

use reqwest::header::AUTHORIZATION;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

use crate::error::CustomError;
use crate::object::{ExpoPushReceipt, GetPushNotificationReceiptsRequest};
use crate::ExpoPushReceiptId;

#[derive(Debug, PartialEq, serde::Deserialize)]
struct GetPushNotificationReceiptsSuccessfulResponse {
    data: HashMap<ExpoPushReceiptId, ExpoPushReceipt>,
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
                Ok(response
                    .json::<GetPushNotificationReceiptsSuccessfulResponse>()
                    .await
                    .map_err(|err| {
                        CustomError::DeserializeErr(format!(
                            "Failed to deserialize response: {}",
                            err
                        ))
                    })?
                    .data)
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
