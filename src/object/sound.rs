use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::skip_serializing_none;

// <https://docs.expo.dev/push-notifications/sending-notifications/#message-request-format>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CriticalSound {
    pub critical: Option<bool>,
    pub name: Option<String>,
    pub volume: Option<f64>,
}

impl CriticalSound {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn critical(mut self, critical: bool) -> Self {
        self.critical = Some(critical);
        self
    }

    pub fn name<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    pub fn volume(mut self, volume: f64) -> Self {
        self.volume = Some(volume);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sound {
    Default,
    Custom(String),
    Critical(CriticalSound),
}

impl Serialize for Sound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Sound::Default => serializer.serialize_str("default"),
            Sound::Custom(s) => serializer.serialize_str(s),
            Sound::Critical(sound) => sound.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Sound {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            Name(String),
            Critical(CriticalSound),
        }

        match Repr::deserialize(deserializer)? {
            Repr::Name(s) if s == "default" => Ok(Sound::Default),
            Repr::Name(s) => Ok(Sound::Custom(s)),
            Repr::Critical(sound) => Ok(Sound::Critical(sound)),
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

    #[test]
    fn test_critical_sound_serialization() -> Result<(), serde_json::Error> {
        let sound = Sound::Critical(
            CriticalSound::new()
                .critical(true)
                .name("bells.wav")
                .volume(0.5),
        );
        assert_eq!(
            serde_json::to_value(&sound)?,
            serde_json::json!({ "critical": true, "name": "bells.wav", "volume": 0.5 })
        );
        Ok(())
    }

    #[test]
    fn test_critical_sound_serialization_skips_unset_fields() -> Result<(), serde_json::Error> {
        let sound = Sound::Critical(CriticalSound::new().critical(true));
        assert_eq!(
            serde_json::to_value(&sound)?,
            serde_json::json!({ "critical": true })
        );
        assert_eq!(
            serde_json::to_value(Sound::Critical(CriticalSound::new()))?,
            serde_json::json!({})
        );
        Ok(())
    }

    #[test]
    fn test_critical_sound_deserialization() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::from_str::<Sound>(r#"{"critical":true,"name":"bells.wav","volume":0.5}"#)?,
            Sound::Critical(
                CriticalSound::new()
                    .critical(true)
                    .name("bells.wav")
                    .volume(0.5)
            )
        );
        assert_eq!(
            serde_json::from_str::<Sound>("{}")?,
            Sound::Critical(CriticalSound::new())
        );
        Ok(())
    }
}
