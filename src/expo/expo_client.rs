mod get;
mod post;

use crate::{
    error::CustomError,
    object::{
        expo_push_message::ExpoPushMessage, expo_push_receipt::ExpoPushReceipt,
        expo_push_receipt_id::ExpoPushReceiptId, expo_push_ticket::ExpoPushTicket,
    },
};

use self::{get::get_push_notification_receipts, post::send_push_notifications};

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
        receipt_id: ExpoPushReceiptId,
    ) -> Result<Vec<ExpoPushReceipt>, CustomError> {
        get_push_notification_receipts(&self.client, receipt_id, self.access_token.as_deref()).await
    }
}
