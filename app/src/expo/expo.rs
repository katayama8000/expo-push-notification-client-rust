use crate::{
    error::CustomError,
    get_push_notification_receipts::{
        get_push_notification_receipts, ExpoPushReceipt, ExpoPushReceiptId,
    },
    send_push_notifications::{send_push_notifications, ExpoPushMessage, ExpoPushTicket},
};

pub struct Expo {
    access_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExpoClientOptions {
    pub access_token: Option<String>,
}

impl Expo {
    pub fn new(options: ExpoClientOptions) -> Self {
        Expo {
            access_token: options.access_token,
        }
    }

    pub fn is_expo_push_token(&self, _token: &str) -> bool {
        unimplemented!()
    }

    pub async fn send_push_notifications(
        &self,
        messages: ExpoPushMessage,
    ) -> Result<Vec<ExpoPushTicket>, CustomError> {
        send_push_notifications(messages, self.access_token.clone()).await
    }

    pub async fn get_push_notification_receipts(
        &self,
        receipt_id: ExpoPushReceiptId,
    ) -> Result<Vec<ExpoPushReceipt>, CustomError> {
        get_push_notification_receipts(receipt_id, self.access_token.clone()).await
    }

    // Other methods and helper functions go here
}

// #[derive(Debug, Clone)]
// pub struct ExpoClientOptions {
//     pub http_agent: reqwest::blocking::Agent,
//     pub max_concurrent_requests: Option<usize>,
//     pub access_token: Option<String>,
// }

// // Define other types and structs as needed

// fn main() {
//     // Example usage of the Expo struct in Rust
//     let options = ExpoClientOptions {
//         http_agent: reqwest::blocking::Agent::new(),
//         max_concurrent_requests: Some(1),
//         access_token: Some("your_access_token".to_string()),
//     };

//     let expo = Expo::new(options);

//     let push_messages = vec![ExpoPushMessage {
//         to: "recipient_token".to_string(),
//         data: Some(HashMap::new()),
//         title: Some("Notification Title".to_string()),
//         // ... other fields ...
//     }];

//     match expo.send_push_notifications(push_messages) {
//         Ok(tickets) => {
//             println!("Push notifications sent successfully: {:?}", tickets);
//         }
//         Err(err) => {
//             eprintln!("Failed to send push notifications: {:?}", err);
//         }
//     }
// }
