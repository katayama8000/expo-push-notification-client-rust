use crate::{
    error::CustomError,
    get_push_notification_receipts::get_push_notification_receipts,
    object::{
        expo_push_message::ExpoPushMessage, expo_push_receipt::ExpoPushReceipt,
        expo_push_receipt_id::ExpoPushReceiptId, expo_push_ticket::ExpoPushTicket,
    },
    send_push_notifications::send_push_notifications,
};

pub struct Expo {
    access_token: Option<String>,
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
            client: reqwest::Client::new(),
        }
    }

    pub fn is_expo_push_token(&self, _token: &str) -> bool {
        unimplemented!()
    }

    pub async fn send_push_notifications(
        &self,
        messages: ExpoPushMessage,
    ) -> Result<Vec<ExpoPushTicket>, CustomError> {
        send_push_notifications(self.client.clone(), messages, self.access_token.clone()).await
    }

    pub async fn get_push_notification_receipts(
        &self,
        receipt_id: ExpoPushReceiptId,
    ) -> Result<Vec<ExpoPushReceipt>, CustomError> {
        get_push_notification_receipts(self.client.clone(), receipt_id, self.access_token.clone())
            .await
    }
}
