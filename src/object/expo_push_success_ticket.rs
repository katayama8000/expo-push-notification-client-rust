use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpoPushSuccessTicket {
    pub id: String,
}
