# expo-server-sdk-rust
Expo Push Notification Rust Client

## client (ReactNative with Expo)
You need to get Expo Push Token from Expo SDK and send it to Expo server first.  
See [Expo Push Notification Docs](https://docs.expo.dev/push-notifications/push-notifications-setup/) for more details.  

## server (Rust)
install expo_server_sdk crate
```bash
cargo add expo_server_sdk
```
how to use 
```rust
use expo_server_sdk::push_message;
push_message(
    "ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]",
    "Title",
    "Body",
);
```