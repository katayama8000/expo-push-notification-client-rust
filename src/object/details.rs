use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// <https://docs.expo.dev/push-notifications/sending-notifications/#push-ticket-errors>
#[skip_serializing_none]
#[derive(Debug, Clone, Eq, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    pub error: Option<DetailsErrorType>,
    pub expo_push_token: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::from_str::<Details>(
                r#"{"error":"DeviceNotRegistered","expoPushToken":"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"}"#
            )?,
            Details {
                error: Some(DetailsErrorType::DeviceNotRegistered),
                expo_push_token: Some("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()),
            }
        );
        assert_eq!(
            serde_json::from_str::<Details>(r#"{"error":"MessageTooBig"}"#)?,
            Details {
                error: Some(DetailsErrorType::MessageTooBig),
                expo_push_token: None,
            }
        );
        Ok(())
    }

    #[test]
    fn test_serialization() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::to_value(Details {
                error: Some(DetailsErrorType::DeviceNotRegistered),
                expo_push_token: Some("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]".to_string()),
            })?,
            serde_json::json!({
                "error": "DeviceNotRegistered",
                "expoPushToken": "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"
            })
        );
        assert_eq!(
            serde_json::to_value(Details {
                error: None,
                expo_push_token: None,
            })?,
            serde_json::json!({})
        );
        Ok(())
    }
}
