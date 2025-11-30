use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InterruptionLevel {
    Active,
    Critical,
    Passive,
    TimeSensitive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Active)?,
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Critical)?,
            "\"critical\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Passive)?,
            "\"passive\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::TimeSensitive)?,
            "\"time-sensitive\""
        );
        Ok(())
    }

    #[test]
    fn test_deserialization() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"active\"")?,
            InterruptionLevel::Active
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"critical\"")?,
            InterruptionLevel::Critical
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"passive\"")?,
            InterruptionLevel::Passive
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"time-sensitive\"")?,
            InterruptionLevel::TimeSensitive
        );
        Ok(())
    }
}
