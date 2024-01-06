use crate::ExpoPushErrorReceipt;

use super::expo_push_success_ticket::ExpoPushSuccessTicket;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum ExpoPushTicket {
    Ok(ExpoPushSuccessTicket),
    Error(ExpoPushErrorReceipt),
}
