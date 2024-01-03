use serde::{Deserialize, Serialize};

use super::{
    expo_push_error_ticket::ExpoPushErrorTicket, expo_push_success_ticket::ExpoPushSuccessTicket,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExpoPushTicket {
    Success(ExpoPushSuccessTicket),
    Error(ExpoPushErrorTicket),
}
