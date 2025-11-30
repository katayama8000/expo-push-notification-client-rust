use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RichContent {
    pub image: Option<String>,
}

impl RichContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn image<S>(mut self, image: S) -> Self
    where
        S: Into<String>,
    {
        self.image = Some(image.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::RichContent;
    use serde_json;

    #[test]
    fn test_serialize() -> Result<(), serde_json::Error> {
        let content = RichContent::new().image("https://example.com/image.png");
        let serialized = serde_json::to_string(&content)?;
        assert_eq!(serialized, r#"{"image":"https://example.com/image.png"}"#);
        Ok(())
    }

    #[test]
    fn test_serialize_none() -> Result<(), serde_json::Error> {
        let content = RichContent::new();
        let serialized = serde_json::to_string(&content)?;
        assert_eq!(serialized, r#"{}"#);
        Ok(())
    }
}
