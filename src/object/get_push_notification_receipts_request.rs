use serde::{Deserialize, Serialize};

use crate::ExpoPushReceiptId;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct GetPushNotificationReceiptsRequest {
    ids: Vec<ExpoPushReceiptId>,
}

impl GetPushNotificationReceiptsRequest {
    pub fn new(ids: Vec<ExpoPushReceiptId>) -> Self {
        Self { ids }
    }
}
