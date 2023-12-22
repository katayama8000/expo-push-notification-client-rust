use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ExpoPushReceiptId {
    pub ids: Vec<String>,
}

impl ExpoPushReceiptId {
    pub fn new(ids: Vec<String>) -> Self {
        ExpoPushReceiptId { ids }
    }
}
