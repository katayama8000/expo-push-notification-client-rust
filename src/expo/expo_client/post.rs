use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
mod response;

use response::SendPushNotificationResponse;

use crate::{
    error::CustomError,
    object::{
        details::Details, expo_push_message::ExpoPushMessage,
        expo_push_success_ticket::ExpoPushSuccessTicket,
    },
    object::{expo_push_error_ticket::ExpoPushErrorTicket, expo_push_ticket::ExpoPushTicket},
};

pub(crate) async fn send_push_notifications(
    client: &reqwest::Client,
    push_message: ExpoPushMessage,
    access_token: Option<&str>,
) -> Result<Vec<ExpoPushTicket>, CustomError> {
    const URL: &str = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(token) = access_token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }

    match client
        .post(URL)
        .headers(headers)
        .json(&push_message)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response
                    .json::<SendPushNotificationResponse>()
                    .await
                    .map_err(|err| {
                        CustomError::DeserializeErr(format!(
                            "Failed to deserialize response: {:?}",
                            err
                        ))
                    })?
                    .data
                    .into_iter()
                    .map(|item| {
                        if item.status == "error" {
                            ExpoPushTicket::Error(ExpoPushErrorTicket {
                                message: item.message.expect("message is empty"),
                                details: item.details,
                            })
                        } else if item.status == "ok" {
                            ExpoPushTicket::Success(ExpoPushSuccessTicket {
                                id: item.id.expect("id is empty"),
                            })
                        } else {
                            unreachable!("Unknown status: {}", item.status)
                        }
                    })
                    .collect())
            } else {
                Err(CustomError::ServerErr(format!(
                    "Failed to send request: {:?} ===> 1",
                    response
                )))
            }
        }
        Err(err) => Err(CustomError::ServerErr(format!(
            "Failed to send request: {:?} ===> 2",
            err
        ))),
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore]
    async fn test_valid_post() {
        todo!("test")
    }

    #[tokio::test]
    #[ignore]
    async fn test_invalid_post() {
        todo!("test")
    }
}
