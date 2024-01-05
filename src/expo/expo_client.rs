mod get;
mod post;

use std::collections::HashMap;

use crate::{
    error::CustomError,
    object::{
        ExpoPushMessage, ExpoPushReceipt, ExpoPushTicket, GetPushNotificationReceiptsRequest,
    },
    ExpoPushReceiptId,
};

use self::{get::get_push_notification_receipts, post::send_push_notifications};

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
        receipt_id: GetPushNotificationReceiptsRequest,
    ) -> Result<HashMap<ExpoPushReceiptId, ExpoPushReceipt>, CustomError> {
        get_push_notification_receipts(
            &self.base_url,
            &self.client,
            receipt_id,
            self.access_token.as_deref(),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use crate::ExpoPushSuccessReceipt;

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
                ExpoPushReceipt::Success(ExpoPushSuccessReceipt),
            );
            map.insert(
                ExpoPushReceiptId::from_str("ZZZZZZZZ-ZZZZ-ZZZZ-ZZZZ-ZZZZZZZZZZZZ")?,
                ExpoPushReceipt::Success(ExpoPushSuccessReceipt),
            );
            map
        });
        mock.assert();
        Ok(())
    }
}
