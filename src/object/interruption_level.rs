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
    fn test_serialization() {
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Critical).unwrap(),
            "\"critical\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::Passive).unwrap(),
            "\"passive\""
        );
        assert_eq!(
            serde_json::to_string(&InterruptionLevel::TimeSensitive).unwrap(),
            "\"time-sensitive\""
        );
    }

    #[test]
    fn test_deserialization() {
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"active\"").unwrap(),
            InterruptionLevel::Active
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"critical\"").unwrap(),
            InterruptionLevel::Critical
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"passive\"").unwrap(),
            InterruptionLevel::Passive
        );
        assert_eq!(
            serde_json::from_str::<InterruptionLevel>("\"time-sensitive\"").unwrap(),
            InterruptionLevel::TimeSensitive
        );
    }
}
