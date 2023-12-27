use crate::Details;

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
pub(super) struct SendPushNotificationResponse {
    pub data: Vec<SendPushNotificationResponseDataItem>,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase", tag = "status")]
pub(super) enum SendPushNotificationResponseDataItem {
    Ok {
        id: String,
    },
    Error {
        message: String,
        details: Option<Details>,
    },
}

#[cfg(test)]
mod tests {
    use crate::DetailsErrorType;

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
                    SendPushNotificationResponseDataItem::Ok {
                        id: "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string(),
                    },
                    SendPushNotificationResponseDataItem::Ok {
                        id: "YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY".to_string(),
                    },
                    SendPushNotificationResponseDataItem::Ok {
                        id: "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ".to_string(),
                    },
                    SendPushNotificationResponseDataItem::Ok {
                        id: "AAAAAAAA-AAAA-AAAA-AAAA-AAAAAAAAAAAA".to_string(),
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
                    SendPushNotificationResponseDataItem::Error {
                        message: "\"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]\" is not a registered push notification recipient".to_string(),
                        details: Some(Details {
                            error: Some(DetailsErrorType::DeviceNotRegistered),
                        })
                    },
                    SendPushNotificationResponseDataItem::Ok {
                        id: "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string(),
                    },
                ]
            }
        );
        Ok(())
    }
}
