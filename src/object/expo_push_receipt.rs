use std::collections::HashMap;

use serde::Deserialize;

use crate::ExpoPushReceiptId;

use super::{
    expo_push_error_receipt::ExpoPushErrorReceipt,
    expo_push_success_receipt::ExpoPushSuccessReceipt,
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ExpoPushReceipt {
    Success(HashMap<ExpoPushReceiptId, ExpoPushSuccessReceipt>),
    Error(Vec<ExpoPushErrorReceipt>),
}
