# Expo Push Notification Client for Rust

This is an [official](https://docs.expo.dev/push-notifications/sending-notifications/#send-push-notifications-using-a-server) Expo Push Notification Client for Rust.

[![ci](https://github.com/katayama8000/expo-push-notification-client-rust/workflows/ci/badge.svg)](https://github.com/katayama8000/expo-push-notification-client-rust/actions)
[![crates.io](https://img.shields.io/crates/v/expo_push_notification_client)](https://crates.io/crates/expo_push_notification_client)
[![docs.rs](https://img.shields.io/docsrs/expo_push_notification_client)](https://docs.rs/expo_push_notification_client)
[![license](https://img.shields.io/crates/l/expo_push_notification_client)](LICENSE)

## Client (ReactNative with Expo)

You need to get Expo Push Token from Expo SDK and send it to Expo server first.
See [docs](https://docs.expo.dev/push-notifications/push-notifications-setup/) for more details.

## Server (Rust)

### Install

```bash
cargo add expo_push_notification_client
```

### Usage

```rust
use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, GetPushNotificationReceiptsRequest};

// Initialize Expo client
let expo = Expo::new(ExpoClientOptions {
    access_token: Some(access_token),
    use_fcm_v1: Some(false), // Set to true to use FCM v1 API
});

// Define Expo Push Tokens to send notifications to
let expo_push_tokens = ["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"];

// Build Expo Push Message with specified tokens
let expo_push_message = ExpoPushMessage::builder(expo_push_tokens).build()?;

// Send push notifications using Expo client
expo.send_push_notifications(expo_push_message).await;

// Define push notification IDs to retrieve receipts
let expo_push_ids = GetPushNotificationReceiptsRequest::new(vec!["xxxxx".to_string(), "xxxxx".to_string()]);

// Retrieve push notification receipts using Expo client
expo.get_push_notification_receipts(expo_push_ids).await;

```
Additionally, you can further customize the ExpoPushMessage by adding more options. Refer to the [docs](https://docs.expo.dev/push-notifications/sending-notifications/#formats) for more details.
```rust
// Build Expo Push Message with detailed configurations
let expo_push_message = ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"])
    .body("body")
    .data(&[("data".to_string())])?
    .ttl(100)
    .expiration(100)
    .priority("high")
    .subtitle("subtitle")
    .sound("default")
    .badge(1)
    .channel_id("channel_id")
    .category_id("category_id")
    .mutable_content(true)
    .title("title")
    .build()?;
```
