#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ExpoPushReceiptId(String);

impl std::convert::From<ExpoPushReceiptId> for String {
    fn from(expo_push_receipt_id: ExpoPushReceiptId) -> Self {
        expo_push_receipt_id.0
    }
}

impl std::convert::TryFrom<&str> for ExpoPushReceiptId {
    type Error = crate::CustomError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(s.to_string()))
    }
}

impl std::convert::TryFrom<String> for ExpoPushReceiptId {
    type Error = crate::CustomError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(s))
    }
}

impl std::fmt::Display for ExpoPushReceiptId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ExpoPushReceiptId {
    type Err = crate::CustomError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        clone::Clone,
        cmp::Eq,
        fmt::{Debug, Display},
        hash::Hash,
        str::FromStr,
    };

    use super::*;

    #[test]
    fn test_impls() -> anyhow::Result<()> {
        fn assert_clone<T: Clone>() {}
        fn assert_debug<T: Debug>() {}
        fn assert_deserialize<'a, T: serde::Deserialize<'a>>() {}
        fn assert_deserialize_owned<T: serde::de::DeserializeOwned>() {}
        fn assert_display<T: Display>() {}
        fn assert_eq<T: Eq>() {}
        fn assert_from_str<T: FromStr>() {}
        fn assert_hash<T: Hash>() {}
        fn assert_ord<T: Ord>() {}
        fn assert_partial_eq<T: PartialEq>() {}
        fn assert_partial_ord<T: PartialOrd>() {}
        fn assert_send<T: Send>() {}
        fn assert_serialize<T: serde::Serialize>() {}
        fn assert_sync<T: Sync>() {}
        fn assert_try_from<T: TryFrom<String>>() {}
        assert_clone::<ExpoPushReceiptId>();
        assert_debug::<ExpoPushReceiptId>();
        assert_deserialize::<ExpoPushReceiptId>();
        assert_deserialize_owned::<ExpoPushReceiptId>();
        assert_display::<ExpoPushReceiptId>();
        assert_eq::<ExpoPushReceiptId>();
        assert_from_str::<ExpoPushReceiptId>();
        assert_hash::<ExpoPushReceiptId>();
        assert_ord::<ExpoPushReceiptId>();
        assert_partial_eq::<ExpoPushReceiptId>();
        assert_partial_ord::<ExpoPushReceiptId>();
        assert_send::<ExpoPushReceiptId>();
        assert_serialize::<ExpoPushReceiptId>();
        assert_sync::<ExpoPushReceiptId>();
        assert_try_from::<ExpoPushReceiptId>();
        Ok(())
    }

    #[test]
    fn test_string_conversion() -> anyhow::Result<()> {
        let s = "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX";
        assert_eq!(String::from(ExpoPushReceiptId::try_from(s.to_string())?), s);
        assert_eq!(format!("{}", ExpoPushReceiptId::from_str(s)?), s);
        Ok(())
    }
}
