use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::{From, TryFrom, TryInto};

use crate::be_bytes::BeBytes;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EthereumType<const N: usize>([u8; N]);

impl<const N: usize> Serialize for EthereumType<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, const N: usize> Deserialize<'de> for EthereumType<N> {
    fn deserialize<D>(deserializer: D) -> Result<EthereumType<N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(EthereumTypeVisitor::<N>)
    }
}

struct EthereumTypeVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for EthereumTypeVisitor<N> {
    type Value = EthereumType<N>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a hex string")
    }

    fn visit_str<T: serde::de::Error>(self, value: &str) -> Result<Self::Value, T> {
        let result = Self::Value::try_from(value)
            .map_err(|e| T::custom(format!("Deserialization error: {:?}", e)))?;
        Ok(result)
    }

    fn visit_string<T: serde::de::Error>(self, value: String) -> Result<Self::Value, T> {
        let result = Self::Value::try_from(value.as_str())
            .map_err(|e| T::custom(format!("Deserialization error: {:?}", e)))?;
        Ok(result)
    }
}

impl<const N: usize> EthereumType<N> {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn into_bytes(self) -> [u8; N] {
        self.0
    }

    #[inline]
    pub fn zero() -> Self {
        Self([0_u8; N])
    }

    #[inline]
    pub fn try_from_int<const L: usize>(value: impl BeBytes<L>) -> Result<Self, ConversionError> {
        if N >= L {
            let mut data = [0_u8; N];
            data[N - L..].copy_from_slice(&value.be_bytes()[..]);
            Ok(Self(data))
        } else {
            Err(ConversionError::TryFromIntError(format!(
                "input does not fit into {} bytes",
                N
            )))
        }
    }

    /// # Panics
    ///
    /// Panics if the input data doesn't fit into the type, i.e. `N < L`.
    #[inline]
    pub fn from_int_unchecked<const L: usize>(value: impl BeBytes<L>) -> Self {
        let mut data = [0_u8; N];
        data[N - L..].copy_from_slice(&value.be_bytes()[..]);
        Self(data)
    }
}

impl<const N: usize> std::fmt::Display for EthereumType<N> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex_string = self
            .0
            .iter()
            .skip_while(|&x| N != 20 && x == &0_u8) // address should not have its leading zeros removed
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("");

        write!(
            formatter,
            "0x{}",
            if N != 20 && !hex_string.is_empty() {
                // remove remaining leading zero (e.g. 7 will be formatted as 0x07)
                hex_string.as_str().trim_start_matches('0')
            } else if hex_string.is_empty() {
                // address (N = 20) cannot be empty by default
                // if we have EthereumType::zero(), just add a 0
                "0"
            } else {
                hex_string.as_str()
            }
        )
    }
}

impl<const N: usize> Default for EthereumType<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> TryFrom<&[u8]> for EthereumType<N> {
    type Error = ConversionError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|_| {
            ConversionError::TryFromSliceError(format!(
                "input length was {}, expected {}",
                value.len(),
                N
            ))
        })?))
    }
}

impl<const N: usize> From<[u8; N]> for EthereumType<N> {
    #[inline]
    fn from(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<&[u8; N]> for EthereumType<N> {
    #[inline]
    fn from(value: &[u8; N]) -> Self {
        let mut data = [0u8; N];
        data.copy_from_slice(value);
        Self(data)
    }
}

impl<const N: usize> TryFrom<&str> for EthereumType<N> {
    type Error = ConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let stripped = value.trim_start_matches("0x");
        let length = stripped.len();
        if length <= 2 * N {
            let mut data = [0_u8; N];
            let end = if length % 2 == 0 {
                length / 2
            } else {
                length / 2 + 1
            };
            let mut stripped_rev = stripped.chars().rev();
            for i in 0..end {
                let first = stripped_rev.next().unwrap().to_digit(16).ok_or_else(|| {
                    ConversionError::TryFromStrError("invalid digit found in string".to_owned())
                })?;
                let second = if let Some(sec) = stripped_rev.next() {
                    sec.to_digit(16).ok_or_else(|| {
                        ConversionError::TryFromStrError("invalid digit found in string".to_owned())
                    })?
                } else {
                    0
                };
                data[N - 1 - i] = (first + second * 16) as u8;
            }
            Ok(Self(data))
        } else {
            Err(ConversionError::TryFromStrError(format!(
                "input does not fit into {} bytes",
                N
            )))
        }
    }
}

#[derive(Debug)]
pub enum ConversionError {
    TryFromSliceError(String),
    TryFromStrError(String),
    TryFromIntError(String),
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn try_from_str() {
        let test_str = "0x1234567890abcdeffedcba098765432100007777";
        let non_prefixed_string =
            EthereumType::<20>::try_from(test_str.strip_prefix("0x").unwrap())
                .unwrap()
                .to_string();
        let zerox_prefixed_string = EthereumType::<20>::try_from(test_str).unwrap().to_string();

        assert_eq!(non_prefixed_string, test_str.to_owned());
        assert_eq!(zerox_prefixed_string, test_str.to_owned());

        let test_str = "1234567890abcdeffedcba09876543210000777";
        let address = EthereumType::<20>::try_from(test_str).unwrap().to_string();
        assert_eq!(address, "0x01234567890abcdeffedcba09876543210000777"); // note the leading zero

        let address = EthereumType::<20>::try_from("0x12345").unwrap().to_string();
        assert_eq!(address, "0x0000000000000000000000000000000000012345"); // note the leading zero

        let test_str = "1234567";
        let eth = EthereumType::<8>::try_from(test_str).unwrap().to_string();
        assert_eq!(eth, "0x1234567");

        let test_str = "7";
        let eth = EthereumType::<1>::try_from(test_str).unwrap().to_string();
        assert_eq!(eth, "0x7");
    }

    #[test]
    fn try_from_invalid_str() {
        // data too long
        let test_str = "0x1234567890abcdeffedcba0987654321000077778";
        let eth = EthereumType::<20>::try_from(test_str);
        assert!(eth.is_err());
        if let Err(ConversionError::TryFromStrError(err_msg)) = eth {
            assert_eq!(err_msg, "input does not fit into 20 bytes".to_owned());
        } else {
            panic!("should be a TryFromStrError!")
        }

        // cannot parse `zz` into a hexadecimal number
        let test_str = "0x1234567890abcdeffedcba0987654321000077zz";
        let eth = EthereumType::<20>::try_from(test_str);
        assert!(eth.is_err());
        if let Err(ConversionError::TryFromStrError(err_msg)) = eth {
            assert_eq!(err_msg, "invalid digit found in string".to_owned());
        } else {
            panic!("should be a TryFromStrError!")
        }
    }

    #[test]
    fn try_u256_from_integer() {
        let uint = EthereumType::<32>::try_from_int(123_u8).unwrap();
        assert_eq!(uint.to_string(), "0x7b");

        let int = EthereumType::<32>::try_from_int(-123_i8).unwrap();
        assert_eq!(int.to_string(), "0x85");

        let uint = EthereumType::<32>::try_from_int(0x45fa_u16).unwrap();
        assert_eq!(uint.to_string(), "0x45fa");

        let int = EthereumType::<32>::try_from_int(-0x45fa_i16).unwrap();
        assert_eq!(int.to_string(), "0xba06");

        let uint = EthereumType::<32>::try_from_int(0x2bc45fa_u32).unwrap();
        assert_eq!(uint.to_string(), "0x2bc45fa");

        let int = EthereumType::<32>::try_from_int(-0x2bc45fa_i32).unwrap();
        assert_eq!(int.to_string(), "0xfd43ba06");

        let uint = EthereumType::<32>::try_from_int(0xfff2bc45fa_u64).unwrap();
        assert_eq!(uint.to_string(), "0xfff2bc45fa");

        let uint = EthereumType::<32>::try_from_int(0xbbdeccaafff2bc45fa_u128).unwrap();
        assert_eq!(uint.to_string(), "0xbbdeccaafff2bc45fa");

        let uint = EthereumType::<32>::from_int_unchecked(0xbbdeccaafff2bc45fa_u128);
        assert_eq!(uint.to_string(), "0xbbdeccaafff2bc45fa");

        let uint = EthereumType::<4>::from_int_unchecked(0x5fa_u16);
        assert_eq!(uint.to_string(), "0x5fa");
    }

    #[test]
    fn try_from_slice() {
        let eth = EthereumType::<16>::try_from(vec![1_u8, 2, 3, 4, 5, 6].as_slice());
        if let Err(ConversionError::TryFromSliceError(err_msg)) = eth {
            assert_eq!(err_msg, "input length was 6, expected 16");
        } else {
            panic!("should be a TryFromSliceError");
        }
    }

    #[test]
    fn try_from_too_large_integer() {
        let uint = EthereumType::<0>::try_from_int(123_u8);
        assert!(uint.is_err());

        let uint = EthereumType::<8>::try_from_int(0xbbdeccaafff2bc45fa_u128);
        assert!(uint.is_err());
        if let Err(ConversionError::TryFromIntError(err_msg)) = uint {
            assert_eq!(err_msg, "input does not fit into 8 bytes".to_owned());
        } else {
            panic!("should be a TryFromIntError!")
        }
    }

    #[test]
    #[should_panic]
    fn from_invalid_unchecked() {
        EthereumType::<8>::from_int_unchecked(0xbbdeccaafff2bc45fa_u128);
    }

    #[test]
    fn zero_array() {
        assert_eq!(EthereumType::<8>::zero().into_bytes(), [0_u8; 8]);
    }

    #[test]
    fn from_valid_fixed_array() {
        EthereumType::<1>::from([12_u8]);
        EthereumType::<2>::from([12_u8; 2]);
        EthereumType::<20>::from([12_u8; 20]);
        EthereumType::<32>::from([12_u8; 32]);
        EthereumType::<32>::from(&[12_u8; 32]);
        let eth = EthereumType::<64>::from([12_u8; 64]);
        assert_eq!(eth.into_bytes(), [12_u8; 64]);
        let eth = EthereumType::<64>::from(&[12_u8; 64]);
        assert_eq!(eth.into_bytes(), [12_u8; 64]);
    }

    #[test]
    fn serde_tests() {
        let eth = EthereumType::<8>::try_from("0x456abcf").unwrap();
        let expected = "0x456abcf";
        serde_test::assert_tokens(&eth, &[serde_test::Token::BorrowedStr(expected)]);

        let eth = EthereumType::<4>::try_from("0xffaabb1").unwrap();
        let expected = "0xffaabb1";
        serde_test::assert_tokens(&eth, &[serde_test::Token::BorrowedStr(expected)]);
    }

    #[test]
    fn zero_bloom() {
        let bloom = EthereumType::<256>::zero();
        assert_eq!(bloom.to_string(), "0x0".to_owned());
    }

    #[test]
    fn zero_address() {
        let address = EthereumType::<20>::zero();
        assert_eq!(
            address.to_string(),
            "0x0000000000000000000000000000000000000000".to_owned()
        );
    }
}
