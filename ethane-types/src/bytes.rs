use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// A type for hex values of arbitrary length
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn from_slice(slice: &[u8]) -> Self {
        Bytes(slice.to_vec())
    }
}

impl Serialize for Bytes {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        serializer.serialize_str(&(String::from("0x") + &hex::encode(&self.0)))
    }
}

impl FromStr for Bytes {
    type Err = hex::FromHexError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim_start_matches("0x");
        Ok(Bytes(hex::decode(trimmed)?))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<T>(deserializer: T) -> Result<Bytes, T::Error>
    where
        T: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a hex string")
    }

    fn visit_str<T: serde::de::Error>(self, value: &str) -> Result<Self::Value, T> {
        let result = Self::Value::from_str(value)
            .map_err(|err| serde::de::Error::custom(format!("Invalid hex string: {}", err)))?;
        Ok(result)
    }

    fn visit_string<T: serde::de::Error>(self, value: String) -> Result<Self::Value, T> {
        self.visit_str(&value)
    }
}

#[test]
fn test_bytes() {
    let bytes_0 = Bytes::from_slice(&[]);
    let bytes_1 = Bytes::from_slice(&[0, 0]);
    let bytes_2 = Bytes::from_slice(&[17, 234]);
    let bytes_3 = Bytes::from_str("0x").unwrap();
    let bytes_4 = Bytes::from_str("0x00").unwrap();
    let bytes_5 = Bytes::from_str("00421100").unwrap();

    let expected_0 = "0x";
    let expected_1 = "0x0000";
    let expected_2 = "0x11ea";
    let expected_3 = Bytes(vec![]);
    let expected_4 = Bytes(vec![0]);
    let expected_5 = Bytes(vec![0, 66, 17, 0]);

    serde_test::assert_tokens(&bytes_0, &[serde_test::Token::BorrowedStr(expected_0)]);
    serde_test::assert_tokens(&bytes_1, &[serde_test::Token::BorrowedStr(expected_1)]);
    serde_test::assert_tokens(&bytes_2, &[serde_test::Token::BorrowedStr(expected_2)]);
    assert_eq!(bytes_3, expected_3);
    assert_eq!(bytes_4, expected_4);
    assert_eq!(bytes_5, expected_5);
}
