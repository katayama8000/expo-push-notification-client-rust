use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub(super) struct GetPushNotificationReceiptsResponse {
    pub data: HashMap<String, GetPushNotificationReceiptsResponseDataItem>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub(super) struct GetPushNotificationReceiptsResponseDataItem {
    pub status: String,
    pub message: Option<String>,
    pub details: Option<GetPushNotificationReceiptsResponseDataItemDetails>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
pub(super) struct GetPushNotificationReceiptsResponseDataItemDetails {
    pub error: Option<PushReceiptError>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
pub(super) enum PushReceiptError {
    DeviceNotRegistered,
    MessageTooBig,
    MessageRateExceeded,
    MismatchSenderId,
    InvalidCredentials,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let response_body = r#"
{
  "data": {
    "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX": { "status": "ok" },
    "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ": { "status": "ok" }
  }
}
"#;
        let parsed = serde_json::from_str::<GetPushNotificationReceiptsResponse>(response_body)?;
        assert_eq!(
            parsed,
            GetPushNotificationReceiptsResponse {
                data: {
                    let mut map = HashMap::new();
                    map.insert(
                        "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string(),
                        GetPushNotificationReceiptsResponseDataItem {
                            status: "ok".to_string(),
                            message: None,
                            details: None,
                        },
                    );
                    map.insert(
                        "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ".to_string(),
                        GetPushNotificationReceiptsResponseDataItem {
                            status: "ok".to_string(),
                            message: None,
                            details: None,
                        },
                    );
                    map
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_contains_error() -> anyhow::Result<()> {
        let response_body = r#"
{
  "data": {
    "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX": { "status": "ok" },
    "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ": {
      "status": "error",
      "message": "...",
      "details": {
        "error": "DeviceNotRegistered"
      }
    },
    "AAAAAAAA-AAAA-AAAA-AAAA-AAAAAAAAAAAA": {
      "status": "error",
      "message": "...",
      "details": {
        "error": "MessageTooBig"
      }
    },
    "BBBBBBBB-BBBB-BBBB-BBBB-BBBBBBBBBBBB": {
      "status": "error",
      "message": "...",
      "details": {
        "error": "MessageRateExceeded"
      }
    },
    "CCCCCCCC-CCCC-CCCC-CCCC-CCCCCCCCCCCC": {
      "status": "error",
      "message": "...",
      "details": {
        "error": "MismatchSenderId"
      }
    },
    "DDDDDDDD-DDDD-DDDD-DDDD-DDDDDDDDDDDD": {
      "status": "error",
      "message": "...",
      "details": {
        "error": "InvalidCredentials"
      }
    }
  }
}
"#;
        let parsed = serde_json::from_str::<GetPushNotificationReceiptsResponse>(response_body)?;
        assert_eq!(
            parsed,
            GetPushNotificationReceiptsResponse {
                data: {
                    let mut map = HashMap::new();
                    map.insert(
                        "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string(),
                        GetPushNotificationReceiptsResponseDataItem {
                            status: "ok".to_string(),
                            message: None,
                            details: None,
                        },
                    );
                    map.insert(
                        "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ".to_string(),
                        GetPushNotificationReceiptsResponseDataItem {
                            status: "error".to_string(),
                            message: Some("...".to_string()),
                            details: Some(GetPushNotificationReceiptsResponseDataItemDetails {
                                error: Some(PushReceiptError::DeviceNotRegistered),
                            }),
                        },
                    );
                    map.insert(
                        "AAAAAAAA-AAAA-AAAA-AAAA-AAAAAAAAAAAA".to_string(),
                        GetPushNotificationReceiptsResponseDataItem {
                            status: "error".to_string(),
                            message: Some("...".to_string()),
                            details: Some(GetPushNotificationReceiptsResponseDataItemDetails {
                                error: Some(PushReceiptError::MessageTooBig),
                            }),
                        },
                    );
                    map.insert(
                        "BBBBBBBB-BBBB-BBBB-BBBB-BBBBBBBBBBBB".to_string(),
                        GetPushNotificationReceiptsResponseDataItem {
                            status: "error".to_string(),
                            message: Some("...".to_string()),
                            details: Some(GetPushNotificationReceiptsResponseDataItemDetails {
                                error: Some(PushReceiptError::MessageRateExceeded),
                            }),
                        },
                    );
                    map.insert(
                        "CCCCCCCC-CCCC-CCCC-CCCC-CCCCCCCCCCCC".to_string(),
                        GetPushNotificationReceiptsResponseDataItem {
                            status: "error".to_string(),
                            message: Some("...".to_string()),
                            details: Some(GetPushNotificationReceiptsResponseDataItemDetails {
                                error: Some(PushReceiptError::MismatchSenderId),
                            }),
                        },
                    );
                    map.insert(
                        "DDDDDDDD-DDDD-DDDD-DDDD-DDDDDDDDDDDD".to_string(),
                        GetPushNotificationReceiptsResponseDataItem {
                            status: "error".to_string(),
                            message: Some("...".to_string()),
                            details: Some(GetPushNotificationReceiptsResponseDataItemDetails {
                                error: Some(PushReceiptError::InvalidCredentials),
                            }),
                        },
                    );
                    map
                }
            }
        );
        Ok(())
    }
}
