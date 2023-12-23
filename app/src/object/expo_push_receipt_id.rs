use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ExpoPushReceiptId {
    ids: Vec<String>,
}

impl ExpoPushReceiptId {
    pub fn new(ids: Vec<String>) -> Self {
        ExpoPushReceiptId { ids }
    }
}
