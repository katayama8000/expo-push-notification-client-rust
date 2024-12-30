use crate::{CustomError, ExpoPushMessage};

#[derive(Debug, PartialEq, Clone)]
pub struct SendPushNotificationsRequest(Vec<ExpoPushMessage>);

impl SendPushNotificationsRequest {
    pub fn messages(&self) -> Vec<ExpoPushMessage> {
        self.0.clone()
    }
}

impl SendPushNotificationsRequest {
    fn new(messages: Vec<ExpoPushMessage>) -> Result<Self, CustomError> {
        if messages.is_empty() {
            return Err(CustomError::InvalidArgument(
                "messages must not be empty".to_string(),
            ));
        }
        Ok(Self(messages))
    }
}

impl serde::Serialize for SendPushNotificationsRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0.len() == 1 {
            self.0[0].serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

pub trait TryIntoSendPushNotificationsRequest {
    fn try_into_send_push_notifications_request(
        self,
    ) -> Result<SendPushNotificationsRequest, CustomError>;
}

impl TryIntoSendPushNotificationsRequest for ExpoPushMessage {
    fn try_into_send_push_notifications_request(
        self,
    ) -> Result<SendPushNotificationsRequest, CustomError> {
        SendPushNotificationsRequest::new(vec![self])
    }
}

impl<I> TryIntoSendPushNotificationsRequest for I
where
    I: IntoIterator,
    <I as IntoIterator>::Item: TryInto<ExpoPushMessage>,
    <<I as IntoIterator>::Item as TryInto<ExpoPushMessage>>::Error: Into<CustomError>,
{
    fn try_into_send_push_notifications_request(
        self,
    ) -> Result<SendPushNotificationsRequest, CustomError> {
        SendPushNotificationsRequest::new(
            self.into_iter()
                .map(|i| i.try_into().map_err(|e| e.into()))
                .collect::<Result<Vec<ExpoPushMessage>, CustomError>>()?,
        )
    }
}
