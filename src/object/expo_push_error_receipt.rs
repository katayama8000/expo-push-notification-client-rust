use serde::Deserialize;

use super::details::Details;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ExpoPushErrorReceipt {
    pub message: String,
    pub details: Option<Details>,
}
