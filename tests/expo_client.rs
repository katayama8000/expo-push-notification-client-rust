use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage};

#[tokio::test]
async fn test_chunk_push_notifications() -> anyhow::Result<()> {
    let expo = Expo::new(ExpoClientOptions::default());

    let messages: Vec<ExpoPushMessage> = (0..250)
        .map(|i| {
            ExpoPushMessage::builder([format!("ExponentPushToken[{}]", i)])
                .body("Test message")
                .build()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let chunks = expo.chunk_push_notifications(messages);

    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0].len(), 100);
    assert_eq!(chunks[1].len(), 100);
    assert_eq!(chunks[2].len(), 50);

    Ok(())
}
