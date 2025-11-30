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
