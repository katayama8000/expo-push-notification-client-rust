use serde::Deserialize;

use super::expo_push_error_receipt::ExpoPushErrorReceipt;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum ExpoPushReceipt {
    Ok,
    Error(ExpoPushErrorReceipt),
}
