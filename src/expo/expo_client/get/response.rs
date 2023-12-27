use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(super) struct GetPushNotificationReceiptsResponse {
    pub data: HashMap<String, GetPushNotificationReceiptsResponseDataItem>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(super) struct GetPushNotificationReceiptsResponseDataItem {
    pub status: String,
    pub message: Option<String>,
    pub details: Option<Value>,
}
