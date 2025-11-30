use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sound {
    Default,
    Custom(String),
}

impl Serialize for Sound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Sound::Default => serializer.serialize_str("default"),
            Sound::Custom(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for Sound {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "default" {
            Ok(Sound::Default)
        } else {
            Ok(Sound::Custom(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() -> Result<(), serde_json::Error> {
        assert_eq!(serde_json::to_string(&Sound::Default)?, "\"default\"");
        assert_eq!(
            serde_json::to_string(&Sound::Custom("bells.wav".to_string()))?,
            "\"bells.wav\""
        );
        assert_eq!(
            serde_json::to_string(&Sound::Custom("custom_sound.mp3".to_string()))?,
            "\"custom_sound.mp3\""
        );
        Ok(())
    }

    #[test]
    fn test_deserialization() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::from_str::<Sound>("\"default\"")?,
            Sound::Default
        );
        assert_eq!(
            serde_json::from_str::<Sound>("\"bells.wav\"")?,
            Sound::Custom("bells.wav".to_string())
        );
        assert_eq!(
            serde_json::from_str::<Sound>("\"custom_sound.mp3\"")?,
            Sound::Custom("custom_sound.mp3".to_string())
        );
        Ok(())
    }
}
