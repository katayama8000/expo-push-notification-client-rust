# Expo Push Notification Client for Rust

This is a Expo Push Notification Client for Rust. There is [Expo Server SDK](https://github.com/expo/expo-server-sdk-rust) maintained by community but it is not maintained since 2019.

[![ci](https://github.com/katayama8000/expo-server-sdk-rust/workflows/ci/badge.svg)](https://github.com/katayama8000/expo-server-sdk-rust/actions)
[![crates.io](https://img.shields.io/crates/v/expo-server-sdk)](https://crates.io/crates/expo-server-sdk)
[![docs.rs](https://img.shields.io/docsrs/expo-server-sdk)](https://docs.rs/expo-server-sdk)
[![license](https://img.shields.io/crates/l/expo-server-sdk)](LICENSE)

## client (ReactNative with Expo)

You need to get Expo Push Token from Expo SDK and send it to Expo server first.
See [Expo Push Notification Docs](https://docs.expo.dev/push-notifications/push-notifications-setup/) for more details.

## server (Rust)

### install expo_server_sdk crate

```bash
cargo add expo_server_sdk
```

### how to use

```rust
use expo_server_sdk::{Expo, ExpoClientOptions, ExpoPushMessage, GetPushNotificationReceiptsRequest};

let expo = Expo::new(ExpoClientOptions {
    access_token: Some(access_token),
});

let expo_push_tokens = ["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"];
let expo_push_message = ExpoPushMessage::builder(expo_push_tokens).build()?;
expo.send_push_notifications(expo_push_message).await;

let expo_push_ids = GetPushNotificationReceiptsRequest::new(vec!["xxxxx".to_string(), "xxxxx".to_string()]);
expo.get_push_notification_receipts(expo_push_ids).await;
```
