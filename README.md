# expo-server-sdk-rust
This is a Expo Push Notification Rust Client. There is [Expo Push Notification Rust Client](https://github.com/expo/expo-server-sdk-rust) maintained by community but it is not maintained since 2019.

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
match push_message(push_token, title, body).await {
    Ok(_) => println!("success"),
    Err(e) => println!("error: {}", e),
}
```