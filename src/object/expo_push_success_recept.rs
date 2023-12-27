use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ExpoPushSuccessReceipt {
    pub status: String,
}
