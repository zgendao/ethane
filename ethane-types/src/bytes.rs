use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

/// A type for hex values of arbitrary length
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn from_slice(slice: &[u8]) -> Self {
        Bytes(slice.to_vec())
    }
}

impl TryFrom<&str> for Bytes {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trimmed = value.trim_start_matches("0x");
        let length = trimmed.len();
        let end = if length % 2 == 0 {
            length / 2
        } else {
            length / 2 + 1
        };
        let mut data = Vec::<u8>::with_capacity(end);
        let mut trimmed_chars = trimmed.chars();
        for _ in 0..end {
            let first = trimmed_chars
                .next()
                .unwrap()
                .to_digit(16)
                .ok_or_else(|| String::from("invalid digit found in string"))?;
            let second = if let Some(sec) = trimmed_chars.next() {
                sec.to_digit(16)
                    .ok_or_else(|| String::from("invalid digit found in string"))?
            } else {
                0
            };
            data.push((first * 16 + second) as u8)
        }
        Ok(Self(data))
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "0x{}",
            self.0
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Serialize for Bytes {
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        serializer.serialize_str(&self.to_string())
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
        let result = Self::Value::try_from(value)
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
    let bytes_3 = Bytes::try_from("0x").unwrap();
    let bytes_4 = Bytes::try_from("0x00").unwrap();
    let bytes_5 = Bytes::try_from("00421100").unwrap();

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
