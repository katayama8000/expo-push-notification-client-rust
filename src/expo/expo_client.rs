mod post;

use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Method,
};

use crate::{
    error::CustomError,
    object::{
        ExpoPushMessage, ExpoPushReceipt, ExpoPushTicket, GetPushNotificationReceiptsRequest,
    },
    ExpoPushReceiptId,
};

use self::post::send_push_notifications;

#[derive(Clone)]
pub struct Expo {
    access_token: Option<String>,
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct ExpoClientOptions {
    pub access_token: Option<String>,
}

impl Expo {
    pub fn new(options: ExpoClientOptions) -> Self {
        Expo {
            access_token: options.access_token,
            base_url: "https://exp.host".to_string(),
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

    pub async fn send_push_notifications(
        &self,
        messages: ExpoPushMessage,
    ) -> Result<Vec<ExpoPushTicket>, CustomError> {
        send_push_notifications(
            &self.base_url,
            &self.client,
            messages,
            self.access_token.as_deref(),
        )
        .await
    }

    pub async fn get_push_notification_receipts(
        &self,
        request: GetPushNotificationReceiptsRequest,
    ) -> Result<HashMap<ExpoPushReceiptId, ExpoPushReceipt>, CustomError> {
        #[derive(Debug, PartialEq, serde::Deserialize)]
        struct GetPushNotificationReceiptsSuccessfulResponse {
            data: HashMap<ExpoPushReceiptId, ExpoPushReceipt>,
        }
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

    use crate::{Details, DetailsErrorType, ExpoPushErrorReceipt};

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

        let expo = Expo {
            access_token: None,
            base_url: url,
            client: reqwest::Client::new(),
        };

        let response = expo
            .get_push_notification_receipts(GetPushNotificationReceiptsRequest::new(vec![
                "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string(),
                "YYYYYYYY-YYYY-YYYY-YYYY-YYYYYYYYYYYY".to_string(),
                "ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ".to_string(),
            ]))
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

        let expo = Expo {
            access_token: None,
            base_url: url,
            client: reqwest::Client::new(),
        };

        let response = expo
            .get_push_notification_receipts(GetPushNotificationReceiptsRequest::new(vec![
                "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string(),
            ]))
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

        let expo = Expo {
            access_token: None,
            base_url: url,
            client: reqwest::Client::new(),
        };

        let result = expo
            .get_push_notification_receipts(GetPushNotificationReceiptsRequest::new(vec![
                "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string(),
            ]))
            .await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Server error: Request failed: 401 Unauthorized"
        );
        mock.assert();
        Ok(())
    }
}
