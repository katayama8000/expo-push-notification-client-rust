use super::details::Details;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
pub struct ExpoPushErrorReceipt {
    pub message: String,
    pub details: Option<Details>,
}
