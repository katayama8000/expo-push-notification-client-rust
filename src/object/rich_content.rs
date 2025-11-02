use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RichContent {
    image: Option<String>,
}

impl RichContent {
    pub fn new() -> Self {
        RichContent { image: None }
    }

    pub fn image<S>(mut self, image: S) -> Self
    where
        S: Into<String>,
    {
        self.image = Some(image.into());
        self
    }
}
