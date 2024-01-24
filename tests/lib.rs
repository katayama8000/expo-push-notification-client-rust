use expo_server_sdk::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushReceipt, ExpoPushTicket};

#[ignore = "avoid calling the expo api"]
#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let expo = Expo::new(ExpoClientOptions::default());

    let tokens = ["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"];
    let tickets = expo
        .send_push_notifications(ExpoPushMessage::builder(tokens).build()?)
        .await?;

    let mut ids = vec![];
    for ticket in tickets {
        match ticket {
            ExpoPushTicket::Ok(ticket) => {
                println!("receipt id: {}", ticket.id);
                ids.push(ticket.id);
            }
            ExpoPushTicket::Error(e) => {
                eprintln!("send error: {:?}", e);
            }
        }
    }

    let receipts = expo.get_push_notification_receipts(ids.clone()).await?;

    for id in ids {
        match receipts.get(&id) {
            Some(receipt) => match receipt {
                ExpoPushReceipt::Ok => {
                    println!("receipt ok for id: {}", id);
                }
                ExpoPushReceipt::Error(e) => {
                    eprintln!("get receipt error: {:?}", e);
                }
            },
            None => {
                eprintln!("receipt not found for id: {}", id);
            }
        }
    }

    Ok(())
}
