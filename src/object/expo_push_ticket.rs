use crate::ExpoPushErrorReceipt;

use super::expo_push_success_ticket::ExpoPushSuccessTicket;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub enum ExpoPushTicket {
    Success(ExpoPushSuccessTicket),
    Error(ExpoPushErrorReceipt),
}
