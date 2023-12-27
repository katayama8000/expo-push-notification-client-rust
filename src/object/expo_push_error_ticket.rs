use serde::{Deserialize, Serialize};

use super::details::Details;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpoPushErrorTicket {
    pub status: String,
    pub message: String,
    pub details: Option<Details>,
}
