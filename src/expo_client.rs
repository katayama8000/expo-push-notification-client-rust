use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Method,
};

use crate::{
    error::CustomError,
    object::{ExpoPushReceipt, ExpoPushTicket, TryIntoSendPushNotificationsRequest},
    ExpoPushReceiptId,
};

#[derive(Debug, PartialEq, serde::Deserialize)]
struct SendPushNotificationSuccessfulResponse {
    data: Vec<ExpoPushTicket>,
}

#[derive(Clone)]
pub struct Expo {
    access_token: Option<String>,
    base_url: String,
    client: reqwest::Client,
}

#[derive(Clone, Debug, Default)]
pub struct ExpoClientOptions {
    pub access_token: Option<String>,
    pub base_url: Option<String>,
}

impl Expo {
    pub fn new(options: ExpoClientOptions) -> Self {
        Self {
            access_token: options.access_token,
            base_url: options
                .base_url
                .unwrap_or_else(|| "https://exp.host".to_string()),
            client: reqwest::Client::new(),
        }
    }

    pub fn is_expo_push_token(token: &str) -> bool {
        ((token.starts_with("ExponentPushToken[") || token.starts_with("ExpoPushToken["))
            && token.ends_with(']'))
            || regex::Regex::new(r"^[a-z\d]{8}-[a-z\d]{4}-[a-z\d]{4}-[a-z\d]{4}-[a-z\d]{12}$")
                .expect("regex is valid")
                .is_match(token)
    }

    /// Send push notifications
    ///
    /// <https://docs.expo.dev/push-notifications/sending-notifications/#push-tickets>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # async fn test_send_push_notifications() -> anyhow::Result<()> {
    /// #     use std::str::FromStr as _;
    /// #     use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushReceiptId, ExpoPushSuccessTicket, ExpoPushTicket};
    /// #     let mut server = mockito::Server::new();
    /// #     let url = server.url();
    /// #     let mock = server
    /// #         .mock("POST", "/--/api/v2/push/send")
    /// #         .match_header("content-type", "application/json")
    /// #         .match_body(r#"{"to":["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]}"#)
    /// #         .with_status(200)
    /// #         .with_header("content-type", "application/json; charset=utf-8")
    /// #         .with_body(
    /// #             r#"
    /// # {
    /// #     "data": [
    /// #         { "status": "ok", "id": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX" }
    /// #     ]
    /// # }
    /// # "#,
    /// #         )
    /// #         .create();
    /// #     let expo = Expo::new(ExpoClientOptions {
    /// #         base_url: Some(url),
    /// #         ..Default::default()
    /// #     });
    /// #
    /// let response = expo
    ///     .send_push_notifications(
    ///         ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]).build()?,
    ///     )
    ///     .await?;
    ///
    /// assert_eq!(
    ///     response,
    ///     vec![ExpoPushTicket::Ok(ExpoPushSuccessTicket {
    ///         id: ExpoPushReceiptId::from_str("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?
    ///     })]
    /// );
    /// #     mock.assert();
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn send_push_notifications<R>(
        &self,
        request: R,
    ) -> Result<Vec<ExpoPushTicket>, CustomError>
    where
        R: TryIntoSendPushNotificationsRequest,
    {
        let request = request.try_into_send_push_notifications_request()?;
        let response: SendPushNotificationSuccessfulResponse = self
            .send_request(Method::POST, "/--/api/v2/push/send", request)
            .await?;
        Ok(response.data)
    }

    /// Get push notification receipts
    ///
    ///  <https://docs.expo.dev/push-notifications/sending-notifications/#push-receipts>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # async fn test_get_push_notification_receipts() -> anyhow::Result<()> {
    /// #     use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushReceipt, ExpoPushReceiptId};
    /// #
    /// #     let mut server = mockito::Server::new();
    /// #     let url = server.url();
    /// #     let mock = server
    /// #         .mock("POST", "/--/api/v2/push/getReceipts")
    /// #         .match_header("content-type", "application/json")
    /// #         .match_body(r#"{"ids":["XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"]}"#)
    /// #         .with_status(200)
    /// #         .with_header("content-type", "application/json; charset=utf-8")
    /// #         .with_body(r#"
    /// # {
    /// #   "data": {
    /// #       "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX": { "status": "ok" },
    /// #   }
    /// # }
    /// # "#,
    /// #         )
    /// #         .create();
    /// #     let expo = Expo::new(ExpoClientOptions { base_url: Some(url), ..Default::default() });
    /// let receipt_ids = expo.get_push_notification_receipts([
    ///     "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
    /// ]).await?;
    /// assert!(receipt_ids.contains_key(
    ///     &ExpoPushReceiptId::try_from("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?
    /// ));
    ///
    /// let _ = expo.get_push_notification_receipts(vec![
    ///     ExpoPushReceiptId::try_from("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?
    /// ]).await?;
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn get_push_notification_receipts<I>(
        &self,
        ids: I,
    ) -> Result<HashMap<ExpoPushReceiptId, ExpoPushReceipt>, CustomError>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: TryInto<ExpoPushReceiptId>,
        <<I as IntoIterator>::Item as TryInto<ExpoPushReceiptId>>::Error: Into<CustomError>,
    {
        #[derive(Debug, PartialEq, serde::Serialize)]
        struct GetPushNotificationReceiptsRequest {
            ids: Vec<ExpoPushReceiptId>,
        }
        #[derive(Debug, PartialEq, serde::Deserialize)]
        struct GetPushNotificationReceiptsSuccessfulResponse {
            data: HashMap<ExpoPushReceiptId, ExpoPushReceipt>,
        }
        let ids = ids
            .into_iter()
            .map(|id| id.try_into().map_err(|e| e.into()))
            .collect::<Result<Vec<ExpoPushReceiptId>, CustomError>>()?;
        let request = GetPushNotificationReceiptsRequest { ids };
        let response: GetPushNotificationReceiptsSuccessfulResponse = self
            .send_request(Method::POST, "/--/api/v2/push/getReceipts", request)
            .await?;
        Ok(response.data)
    }

    async fn send_request<S, T>(
        &self,
        method: Method,
        path: &str,
        body: S,
    ) -> Result<T, CustomError>
    where
        S: serde::Serialize,
        T: serde::de::DeserializeOwned,
    {
        let access_token = &self.access_token;
        let base_url = &self.base_url;
        let client = &self.client;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(token) = access_token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
        }

        match client
            .request(method, format!("{}{}", base_url, path))
            .headers(headers)
            .json(&body)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response.json::<T>().await.map_err(|err| {
                        CustomError::DeserializeErr(format!(
                            "Failed to deserialize response: {}",
                            err
                        ))
                    })?)
                } else {
                    Err(CustomError::ServerErr(format!(
                        "Request failed: {}",
                        response.status()
                    )))
                }
            }
            Err(err) => Err(CustomError::ServerErr(format!("Request failed: {}", err))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use crate::{
        Details, DetailsErrorType, ExpoPushErrorReceipt, ExpoPushMessage, ExpoPushSuccessTicket,
    };

    use super::*;

    #[test]
    fn impl_clone_for_expo() {
        fn assert_impl_clone<T: Clone>() {}
        assert_impl_clone::<Expo>();
    }

    #[test]
    fn test_is_expo_push_token() {
        for (s, expected) in [
            ("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]", true),
            ("ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx", false),
            ("ExpoPushToken[xxxxxxxxxxxxxxxxxxxxxx]", true),
            ("ExpoPushToken[xxxxxxxxxxxxxxxxxxxxxx", false),
            ("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", true),
            ("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxx", false),
        ] {
            assert_eq!(
                Expo::is_expo_push_token(s),
                expected,
                "Expo::is_expo_push_token({}) should be {}",
                s,
                expected
            );
        }
    }

    #[tokio::test]
    async fn test_get_push_notification_receipts() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/--/api/v2/push/getReceipts")
            .match_header("content-type", "application/json")
            .match_body(r#"{"ids":["XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX","YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY","ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ"]}"#)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
{
    "data": {
        "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX": { "status": "ok" },
        "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ": { "status": "ok" }
    }
}
"#,
            )
            .create();

        let expo = Expo::new(ExpoClientOptions {
            base_url: Some(url),
            ..Default::default()
        });

        let response = expo
            .get_push_notification_receipts([
                "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX",
                "YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY",
                "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ",
            ])
            .await?;

        assert_eq!(response, {
            let mut map = HashMap::new();
            map.insert(
                ExpoPushReceiptId::from_str("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?,
                ExpoPushReceipt::Ok,
            );
            map.insert(
                ExpoPushReceiptId::from_str("ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ")?,
                ExpoPushReceipt::Ok,
            );
            map
        });
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_get_push_notification_receipts_error_response() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/--/api/v2/push/getReceipts")
            .match_header("content-type", "application/json")
            .match_body(r#"{"ids":["XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"]}"#)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
{
    "data": {
        "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX": {
            "status": "error",
            "message": "\"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]\" is not a registered push notification recipient",
            "details": {
                "error": "DeviceNotRegistered"
            }
        }
    }
}
"#,
            )
            .create();

        let expo = Expo::new(ExpoClientOptions {
            base_url: Some(url),
            ..Default::default()
        });

        let response = expo
            .get_push_notification_receipts(["XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"])
            .await?;

        assert_eq!(response, {
            let mut map = HashMap::new();
            map.insert(
                ExpoPushReceiptId::from_str("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?,
                ExpoPushReceipt::Error(ExpoPushErrorReceipt {
                    message: "\"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]\" is not a registered push notification recipient".to_string(),
                    details: Some(Details { error: Some(DetailsErrorType::DeviceNotRegistered) }),
                }),
            );
            map
        });
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_get_push_notification_receipts_error_response_4xx() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/--/api/v2/push/getReceipts")
            .match_header("content-type", "application/json")
            .match_body(r#"{"ids":["XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"]}"#)
            .with_status(401)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
{
    "error": "invalid_token",
    "error_description":"The bearer token is invalid"
}
"#,
            )
            .create();

        let expo = Expo::new(ExpoClientOptions {
            base_url: Some(url),
            ..Default::default()
        });

        let result = expo
            .get_push_notification_receipts(["XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"])
            .await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Server error: Request failed: 401 Unauthorized"
        );
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_send_push_notifications() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/--/api/v2/push/send")
            .match_header("content-type", "application/json")
            .match_body(r#"{"to":["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]}"#)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
{
    "data": [
        { "status": "ok", "id": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX" }
    ]
}
"#,
            )
            .create();

        let expo = Expo::new(ExpoClientOptions {
            base_url: Some(url),
            ..Default::default()
        });

        let response = expo
            .send_push_notifications(
                ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]).build()?,
            )
            .await?;

        assert_eq!(
            response,
            vec![ExpoPushTicket::Ok(ExpoPushSuccessTicket {
                id: ExpoPushReceiptId::from_str("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?
            })]
        );
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_send_push_notifications_multiple_messages() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/--/api/v2/push/send")
            .match_header("content-type", "application/json")
            .match_body(r#"[{"to":["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]},{"to":["ExponentPushToken[yyyyyyyyyyyyyyyyyyyyyy]"]}]"#)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
{
    "data": [
        { "status": "ok", "id": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX" },
        { "status": "ok", "id": "YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY" }
    ]
}
"#,
            )
            .create();

        let expo = Expo::new(ExpoClientOptions {
            base_url: Some(url),
            ..Default::default()
        });

        let response = expo
            .send_push_notifications([
                ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]).build()?,
                ExpoPushMessage::builder(["ExponentPushToken[yyyyyyyyyyyyyyyyyyyyyy]"]).build()?,
            ])
            .await?;

        assert_eq!(
            response,
            vec![
                ExpoPushTicket::Ok(ExpoPushSuccessTicket {
                    id: ExpoPushReceiptId::from_str("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?
                }),
                ExpoPushTicket::Ok(ExpoPushSuccessTicket {
                    id: ExpoPushReceiptId::from_str("YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY")?
                })
            ]
        );
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_send_push_notifications_error_response() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/--/api/v2/push/send")
            .match_header("content-type", "application/json")
            .match_body(r#"{"to":["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]}"#)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
{
    "data": [
        {
            "status": "error",
            "message": "\"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]\" is not a registered push notification recipient",
            "details": {
                "error": "DeviceNotRegistered"
            }
        }
    ]
}
"#,
            )
            .create();

        let expo = Expo::new(ExpoClientOptions {
            base_url: Some(url),
            ..Default::default()
        });

        let response = expo
            .send_push_notifications(
                ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]).build()?,
            )
            .await?;
        assert_eq!(
            response,
            vec![ExpoPushTicket::Error(ExpoPushErrorReceipt {
                message: r#""ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]" is not a registered push notification recipient"#.to_string(),
                details: Some(Details {
                    error: Some(DetailsErrorType::DeviceNotRegistered),
                })
            })]
        );
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn test_send_push_notifications_4xx() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/--/api/v2/push/send")
            .match_header("content-type", "application/json")
            .match_body(r#"{"to":["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]}"#)
            .with_status(401)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
{
    "error": "invalid_token",
    "error_description":"The bearer token is invalid"
}
"#,
            )
            .create();

        let expo = Expo::new(ExpoClientOptions {
            base_url: Some(url),
            ..Default::default()
        });

        let result = expo
            .send_push_notifications(
                ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]).build()?,
            )
            .await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Server error: Request failed: 401 Unauthorized"
        );
        mock.assert();
        Ok(())
    }

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
        let parsed = serde_json::from_str::<SendPushNotificationSuccessfulResponse>(response_body)?;
        assert_eq!(
            parsed,
            SendPushNotificationSuccessfulResponse {
                data: vec![
                    ExpoPushTicket::Ok(ExpoPushSuccessTicket {
                        id: ExpoPushReceiptId::from_str("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?
                    }),
                    ExpoPushTicket::Ok(ExpoPushSuccessTicket {
                        id: ExpoPushReceiptId::from_str("YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY")?
                    }),
                    ExpoPushTicket::Ok(ExpoPushSuccessTicket {
                        id: ExpoPushReceiptId::from_str("ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ")?,
                    }),
                    ExpoPushTicket::Ok(ExpoPushSuccessTicket {
                        id: ExpoPushReceiptId::from_str("AAAAAAAA-AAAA-AAAA-AAAA-AAAAAAAAAAAA")?,
                    })
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
        let parsed = serde_json::from_str::<SendPushNotificationSuccessfulResponse>(response_body)?;
        assert_eq!(
            parsed,
            SendPushNotificationSuccessfulResponse {
                data: vec![
                    ExpoPushTicket::Error(ExpoPushErrorReceipt {
                        message: "\"ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]\" is not a registered push notification recipient".to_string(),
                        details: Some(Details {
                            error: Some(DetailsErrorType::DeviceNotRegistered),
                        })
                    }),
                    ExpoPushTicket::Ok(ExpoPushSuccessTicket {
                        id: ExpoPushReceiptId::from_str("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")?,
                    }),
                ]
            }
        );
        Ok(())
    }
}
