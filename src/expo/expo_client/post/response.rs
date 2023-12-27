use serde_json::Value;

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
pub(super) struct SendPushNotificationResponse {
    pub data: Vec<SendPushNotificationResponseDataItem>,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
pub(super) struct SendPushNotificationResponseDataItem {
    pub status: String,
    pub id: Option<String>,
    pub message: Option<String>,
    pub details: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_response_body() -> anyhow::Result<()> {
        // <https://docs.expo.dev/push-notifications/sending-notifications/#push-tickets>
        let response_body = r#"
{
  "data": [
    { "status": "ok", "id": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX" },
    { "status": "ok", "id": "YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY" },
    { "status": "ok", "id": "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ" },
    { "status": "ok", "id": "AAAAAAAA-AAAA-AAAA-AAAA-AAAAAAAAAAAA" }
  ]
}
"#;
        let parsed = serde_json::from_str::<SendPushNotificationResponse>(response_body)?;
        assert_eq!(
            parsed,
            SendPushNotificationResponse {
                data: vec![
                    SendPushNotificationResponseDataItem {
                        status: "ok".to_string(),
                        id: Some("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string()),
                        message: None,
                        details: None
                    },
                    SendPushNotificationResponseDataItem {
                        status: "ok".to_string(),
                        id: Some("YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY".to_string()),
                        message: None,
                        details: None
                    },
                    SendPushNotificationResponseDataItem {
                        status: "ok".to_string(),
                        id: Some("ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ".to_string()),
                        message: None,
                        details: None
                    },
                    SendPushNotificationResponseDataItem {
                        status: "ok".to_string(),
                        id: Some("AAAAAAAA-AAAA-AAAA-AAAA-AAAAAAAAAAAA".to_string()),
                        message: None,
                        details: None
                    }
                ]
            }
        );
        Ok(())
    }

    #[test]
    fn test_with_device_not_registerd() -> anyhow::Result<()> {
        // <https://docs.expo.dev/push-notifications/sending-notifications/#push-tickets>
        let response_body = r#"
{
  "data": [
    {
      "status": "error",
      "message": "\"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]\" is not a registered push notification recipient",
      "details": {
        "error": "DeviceNotRegistered"
      }
    },
    {
      "status": "ok",
      "id": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
    }
  ]
}
"#;
        let parsed = serde_json::from_str::<SendPushNotificationResponse>(response_body)?;
        assert_eq!(
            parsed,
            SendPushNotificationResponse {
                data: vec![
                    SendPushNotificationResponseDataItem {
                        status: "error".to_string(),
                        id: None,
                        message: Some("\"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]\" is not a registered push notification recipient".to_string()),
                        details: Some(serde_json::json!({
                          "error": "DeviceNotRegistered"
                        }))
                    },
                    SendPushNotificationResponseDataItem {
                        status: "ok".to_string(),
                        id: Some("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string()),
                        message: None,
                        details: None
                    },
                ]
            }
        );
        Ok(())
    }
}
