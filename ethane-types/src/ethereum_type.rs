use std::array::TryFromSliceError;
use std::convert::{From, TryFrom, TryInto};

//use crate::be_bytes::BeBytes;

pub struct EthereumType<const N: usize>([u8; N]);

impl<const N: usize> EthereumType<N> {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn to_string(&self) -> String {
        format!(
            "0x{}",
            self.0
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .join("")
        )
    }

    #[inline]
    pub fn into_bytes(self) -> [u8; N] {
        self.0
    }
}

/*
impl<const N: usize, const L: usize> TryFrom<&dyn BeBytes<L>> for EthereumType<N> {
    type Error = TryFromPrimitiveError;

    fn try_from(value: &dyn BeBytes<L>) -> Result<Self, Self::Error> {
        if N >= L {
            let mut data = [0_u8; N];
            data[N - L..].copy_from_slice(&value.be_bytes()[..]);
            Ok(Self(data))
        } else {
            Err(TryFromPrimitiveError(format!(
                "Data does not fit into {} bytes",
                N
            )))
        }
    }
}
*/

impl<const N: usize> TryFrom<u8> for EthereumType<N> {
    type Error = TryFromPrimitiveError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if N >= 1 {
            let mut data = [0_u8; N];
            data[N - 1..].copy_from_slice(&value.to_be_bytes()[..]);
            Ok(Self(data))
        } else {
            Err(TryFromPrimitiveError(format!(
                "Data does not fit into {} bytes",
                N
            )))
        }
    }
}

impl<const N: usize> TryFrom<u16> for EthereumType<N> {
    type Error = TryFromPrimitiveError;

    #[inline]
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if N >= 2 {
            let mut data = [0_u8; N];
            data[N - 2..].copy_from_slice(&value.to_be_bytes()[..]);
            Ok(Self(data))
        } else {
            Err(TryFromPrimitiveError(format!(
                "Data does not fit into {} bytes",
                N
            )))
        }
    }
}

impl<const N: usize> TryFrom<u32> for EthereumType<N> {
    type Error = TryFromPrimitiveError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if N >= 4 {
            let mut data = [0_u8; N];
            data[N - 4..].copy_from_slice(&value.to_be_bytes()[..]);
            Ok(Self(data))
        } else {
            Err(TryFromPrimitiveError(format!(
                "Data does not fit into {} bytes",
                N
            )))
        }
    }
}

impl<const N: usize> TryFrom<u64> for EthereumType<N> {
    type Error = TryFromPrimitiveError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if N >= 8 {
            let mut data = [0_u8; N];
            data[N - 8..].copy_from_slice(&value.to_be_bytes()[..]);
            Ok(Self(data))
        } else {
            Err(TryFromPrimitiveError(format!(
                "Data does not fit into {} bytes",
                N
            )))
        }
    }
}

impl<const N: usize> TryFrom<u128> for EthereumType<N> {
    type Error = TryFromPrimitiveError;

    #[inline]
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if N >= 16 {
            let mut data = [0_u8; N];
            data[N - 16..].copy_from_slice(&value.to_be_bytes()[..]);
            Ok(Self(data))
        } else {
            Err(TryFromPrimitiveError(format!(
                "Data does not fit into {} bytes",
                N
            )))
        }
    }
}

impl<const N: usize> TryFrom<&[u8]> for EthereumType<N> {
    type Error = TryFromSliceError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl<const N: usize> From<[u8; N]> for EthereumType<N> {
    #[inline]
    fn from(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<const N: usize> TryFrom<&str> for EthereumType<N> {
    type Error = TryFromStrError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let stripped = if let Some(s) = value.strip_prefix("0x") {
            s
        } else {
            value
        };

        // NOTE only works for even lengths! Should it support types like
        // EthereumType<25> with an odd const generic parameter?
        if stripped.len() == 2 * N {
            let mut data = [0_u8; N];
            for i in 0..stripped.len() / 2 {
                data[i] = u8::from_str_radix(&stripped[2 * i..2 * i + 2], 16)
                    .map_err(|e| TryFromStrError(e.to_string()))?;
            }
            Ok(Self(data))
        } else {
            Err(TryFromStrError(format!(
                "Expected input length was {}, found {}",
                2 * N,
                stripped.len()
            )))
        }
    }
}

#[derive(Debug)]
pub struct TryFromStrError(String);

#[derive(Debug)]
pub struct TryFromPrimitiveError(String);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn try_address_from_str() {
        let test_str = "0x1234567890abcdeffedcba098765432100007777";
        let non_prefixed_address =
            EthereumType::<20>::try_from(test_str.strip_prefix("0x").unwrap()).unwrap();
        let zerox_prefixed_address = EthereumType::<20>::try_from(test_str).unwrap();

        let non_prefixed_string = non_prefixed_address.to_string();
        let zerox_prefixed_string = zerox_prefixed_address.to_string();

        assert_eq!(non_prefixed_string.as_str(), test_str);
        assert_eq!(zerox_prefixed_string.as_str(), test_str);
    }

    #[test]
    fn try_address_from_invalid_str() {
        let test_str = "0x1234567890abcdeffedcba09876543210000777";
        let address = EthereumType::<20>::try_from(test_str);
        assert!(address.is_err());
        assert_eq!(
            address.err().unwrap().0,
            "Expected input length was 40, found 39".to_owned()
        );

        let test_str = "0x1234567890abcdeffedcba0987654321000077778";
        let address = EthereumType::<20>::try_from(test_str);
        assert!(address.is_err());
        assert_eq!(
            address.err().unwrap().0,
            "Expected input length was 40, found 41".to_owned()
        );

        // cannot parse `zz` into a hexadecimal number
        let test_str = "0x1234567890abcdeffedcba0987654321000077zz";
        let address = EthereumType::<20>::try_from(test_str);
        assert!(address.is_err());
        assert_eq!(
            address.err().unwrap().0,
            "invalid digit found in string".to_owned()
        );
    }

    #[test]
    fn try_u256_from_primitives() {
        let uint = EthereumType::<32>::try_from(123_u8).unwrap();
        assert_eq!(
            uint.to_string(),
            "0x000000000000000000000000000000000000000000000000000000000000007b"
        );

        let uint = EthereumType::<32>::try_from(0x45fa_u16).unwrap();
        assert_eq!(
            uint.to_string(),
            "0x00000000000000000000000000000000000000000000000000000000000045fa"
        );

        let uint = EthereumType::<32>::try_from(0x2bc45fa_u32).unwrap();
        assert_eq!(
            uint.to_string(),
            "0x0000000000000000000000000000000000000000000000000000000002bc45fa"
        );

        let uint = EthereumType::<32>::try_from(0xfff2bc45fa_u64).unwrap();
        assert_eq!(
            uint.to_string(),
            "0x000000000000000000000000000000000000000000000000000000fff2bc45fa"
        );

        let uint = EthereumType::<32>::try_from(0xbbdeccaafff2bc45fa_u128).unwrap();
        assert_eq!(
            uint.to_string(),
            "0x0000000000000000000000000000000000000000000000bbdeccaafff2bc45fa"
        );
    }

    #[test]
    fn try_from_too_large_primitive() {
        let uint = EthereumType::<0>::try_from(123_u8);
        assert!(uint.is_err());

        let uint = EthereumType::<8>::try_from(0xbbdeccaafff2bc45fa_u128);
        assert!(uint.is_err());
        assert_eq!(
            uint.err().unwrap().0,
            "Data does not fit into 8 bytes".to_owned()
        );
    }

    #[test]
    fn from_valid_fixed_array() {
        EthereumType::<1>::from([12_u8]);
        EthereumType::<2>::from([12_u8; 2]);
        EthereumType::<20>::from([12_u8; 20]);
        EthereumType::<32>::from([12_u8; 32]);
        EthereumType::<64>::from([12_u8; 64]);
        assert!(true);
    }
}
