mod get;
mod post;

use crate::{
    error::CustomError,
    object::{
        ExpoPushMessage, ExpoPushReceipt, ExpoPushTicket, GetPushNotificationReceiptsRequest,
    },
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
    ) -> Result<Vec<ExpoPushReceipt>, CustomError> {
        get_push_notification_receipts(&self.client, receipt_id, self.access_token.as_deref()).await
    }
}

#[cfg(test)]
mod tests {
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
}
