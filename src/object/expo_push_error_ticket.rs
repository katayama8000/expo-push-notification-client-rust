use serde::{Deserialize, Serialize};

use super::details::Details;
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExpoPushErrorTicket {
    pub message: String,
    pub details: Option<Details>,
}
