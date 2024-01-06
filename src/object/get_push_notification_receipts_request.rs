use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct GetPushNotificationReceiptsRequest {
    ids: Vec<String>,
}

impl GetPushNotificationReceiptsRequest {
    pub fn new(ids: Vec<String>) -> Self {
        Self { ids }
    }
}
