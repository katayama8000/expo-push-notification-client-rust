use serde::Deserialize;

use super::expo_push_error_receipt::ExpoPushErrorReceipt;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ExpoPushReceipt {
    Success,
    Error(ExpoPushErrorReceipt),
}
