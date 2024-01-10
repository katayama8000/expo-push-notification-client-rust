use serde::{Deserialize, Serialize};

use crate::ExpoPushReceiptId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExpoPushSuccessTicket {
    pub id: ExpoPushReceiptId,
}
