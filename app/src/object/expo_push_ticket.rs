use serde::{Deserialize, Serialize};

use super::{
    expo_push_error_ticket::ExpoPushErrorTicket, expo_push_success_ticket::ExpoPushSuccessTicket,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExpoPushTicket {
    Success(ExpoPushSuccessTicket),
    Error(ExpoPushErrorTicket),
}
