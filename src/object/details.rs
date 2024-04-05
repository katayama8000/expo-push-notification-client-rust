use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Deserialize, PartialEq, Serialize)]
pub struct Details {
    pub error: Option<DetailsErrorType>,
}

#[derive(Debug, Clone, Eq, Deserialize, PartialEq, Serialize)]
pub enum DetailsErrorType {
    DeveloperError,
    DeviceNotRegistered,
    ExpoError,
    InvalidCredentials,
    MessageTooBig,
    MessageRateExceeded,
    ProviderError,
}
