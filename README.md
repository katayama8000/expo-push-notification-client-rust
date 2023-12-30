# Expo Push Notification Client for Rust

This is a Expo Push Notification Client for Rust. There is [Expo Server SDK](https://github.com/expo/expo-server-sdk-rust) maintained by community but it is not maintained since 2019.

<!-- [![ci](https://github.com/katayama8000/expo_push_notification_client/workflows/ci/badge.svg)](https://github.com/katayama8000/expo_push_notification_client/actions) -->
[![crates.io](https://img.shields.io/crates/v/expo_push_notification_client)](https://crates.io/crates/expo_push_notification_client)
[![docs.rs](https://img.shields.io/docsrs/expo_push_notification_client)](https://docs.rs/expo_push_notification_client)
[![license](https://img.shields.io/crates/l/expo_push_notification_client)](LICENSE)

## client (ReactNative with Expo)

You need to get Expo Push Token from Expo SDK and send it to Expo server first.
See [Expo Push Notification Docs](https://docs.expo.dev/push-notifications/push-notifications-setup/) for more details.

## server (Rust)

### install expo_push_notification_client crate

```bash
cargo add expo_push_notification_client
```

### how to use

```rust
use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushReceiptId};

let expo = Expo::new(ExpoClientOptions {
    access_token: Some(access_token),
});

let expo_push_tokens = ["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"];
let expo_push_message = ExpoPushMessage::builder(expo_push_tokens).build()?;
expo.send_push_notifications(expo_push_message).await;

let expo_push_ids = ExpoPushReceiptId::new(vec!["xxxxx".to_string(), "xxxxx".to_string()]);
expo.get_push_notification_receipts(expo_push_ids).await;
```
