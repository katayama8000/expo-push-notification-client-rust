use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
mod response;

use response::SendPushNotificationResponse;

use crate::{
    error::CustomError,
    object::{ExpoPushErrorTicket, ExpoPushMessage, ExpoPushSuccessTicket, ExpoPushTicket},
};

pub(crate) async fn send_push_notifications(
    base_url: &str,
    client: &reqwest::Client,
    push_message: ExpoPushMessage,
    access_token: Option<&str>,
) -> Result<Vec<ExpoPushTicket>, CustomError> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(token) = access_token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }

    match client
        .post(format!("{}/--/api/v2/push/send", base_url))
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
                    .map(|item| match item {
                        response::SendPushNotificationResponseDataItem::Ok { id } => {
                            ExpoPushTicket::Success(ExpoPushSuccessTicket { id })
                        }
                        response::SendPushNotificationResponseDataItem::Error {
                            message,
                            details,
                        } => ExpoPushTicket::Error(ExpoPushErrorTicket { message, details }),
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
    use super::*;

    #[tokio::test]
    async fn test_valid_post() -> anyhow::Result<()> {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server
            .mock("POST", "/--/api/v2/push/send")
            .match_header("content-type", "application/json")
            .match_body(r#"{"to":["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]}"#)
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(
                r#"
{
    "data": [
        { "status": "ok", "id": "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX" }
    ]
}
"#,
            )
            .create();

        let response = send_push_notifications(
            &url,
            &reqwest::Client::new(),
            ExpoPushMessage::builder(["ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]"]).build()?,
            None,
        )
        .await?;

        assert_eq!(
            response,
            vec![ExpoPushTicket::Success(ExpoPushSuccessTicket {
                id: "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX".to_string()
            })]
        );
        mock.assert();
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_invalid_post() {
        todo!("test")
    }
}
