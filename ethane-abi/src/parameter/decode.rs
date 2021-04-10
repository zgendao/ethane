use crate::{AbiParserError, Parameter, ParameterType};
use byteorder::{BigEndian, ByteOrder};
use ethereum_types::Address;
use std::convert::TryInto;
use std::str;

impl Parameter {
    /// Attempts to decode a [`Parameter`] based on the supplied
    /// [`ParameterType`] and a raw slice of bytes representing the decoded
    /// value.
    ///
    /// If it succeeds, it returns the decoded [`Parameter`] along with the
    /// number of bytes that encoded its value.
    pub fn decode(
        param_type: &ParameterType,
        raw_bytes: &[u8],
    ) -> Result<(Self, usize), AbiParserError> {
        // TODO validate raw_bytes length
        match *param_type {
            ParameterType::Address => Ok((
                Self::Address(Address::from_slice(remove_left_padding_bytes(
                    12,
                    &raw_bytes[..32],
                ))),
                32,
            )),
            ParameterType::Bool => Ok((Self::Bool(raw_bytes[31] == 1), 32)),
            ParameterType::Uint(length) => match length {
                8 => {
                    let cleaned = remove_left_padding_bytes(32 - (length / 8), &raw_bytes[..32]);
                    Ok((
                        Self::Uint8(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    ))
                }
                16 => {
                    let cleaned = remove_left_padding_bytes(32 - (length / 8), &raw_bytes[..32]);
                    Ok((
                        Self::Uint16(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    ))
                }
                32 => {
                    let cleaned = remove_left_padding_bytes(32 - (length / 8), &raw_bytes[..32]);
                    Ok((
                        Self::Uint32(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    ))
                }
                64 => {
                    let cleaned = remove_left_padding_bytes(32 - (length / 8), &raw_bytes[..32]);
                    Ok((
                        Self::Uint64(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    ))
                }
                128 => {
                    let cleaned = remove_left_padding_bytes(32 - (length / 8), &raw_bytes[..32]);
                    Ok((
                        Self::Uint128(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    ))
                }
                256 => Ok((
                    Self::Uint256(raw_bytes.try_into().map_err(|_| {
                        AbiParserError::InvalidAbiEncoding("Data doesn't fit into type".to_owned())
                    })?),
                    32,
                )),
                _ => unimplemented!(),
            },
            ParameterType::Int(length) => {
                let cleaned = remove_left_padding_bytes(32 - (length / 8), &raw_bytes[..32]);
                match length {
                    8 => Ok((
                        Self::Int8(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    )),
                    16 => Ok((
                        Self::Int16(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    )),
                    32 => Ok((
                        Self::Int32(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    )),
                    64 => Ok((
                        Self::Int64(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    )),
                    128 => Ok((
                        Self::Int128(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    )),
                    256 => Ok((
                        Self::Int256(cleaned.try_into().map_err(|_| {
                            AbiParserError::InvalidAbiEncoding(
                                "Data doesn't fit into type".to_owned(),
                            )
                        })?),
                        32,
                    )),
                    _ => unimplemented!(),
                }
            }
            ParameterType::Bytes => {
                let length = BigEndian::read_u64(&raw_bytes[..32]) as usize;
                Ok((
                    Self::Bytes(raw_bytes[12..length].to_vec()),
                    32 + length + get_right_padding_length(length),
                ))
            }
            ParameterType::String => {
                let length = BigEndian::read_u64(&raw_bytes[..32]) as usize;
                Ok((
                    Self::String(str::from_utf8(&raw_bytes[12..length]).unwrap().to_string()),
                    32 + length + get_right_padding_length(length),
                ))
            }
            ParameterType::FixedBytes(length) => {
                Ok((Self::FixedBytes(raw_bytes[..length].to_vec()), 32))
            }
        }
    }
}

#[inline]
fn remove_left_padding_bytes(pad_length: usize, value: &[u8]) -> &[u8] {
    &value[pad_length..]
}

#[inline]
fn get_right_padding_length(mut length: usize) -> usize {
    let mut res: usize = 0;
    while length % 32 != 0 {
        length += 1;
        res += 1;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use ethereum_types::U256;
    use hex_literal::hex;
    use std::str::FromStr;

    #[test]
    fn test_decode_address() {
        let encoded_address =
            hex!("00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a1234");
        let decoded = Parameter::decode(&ParameterType::Address, &encoded_address).unwrap();

        let expected = Parameter::Address(
            Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
        );
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u8() {
        let encoded_address =
            hex!("000000000000000000000000000000000000000000000000000000000000007F");
        let decoded = Parameter::decode(&ParameterType::Uint(8), &encoded_address).unwrap();
        let expected = Parameter::from_u8(127);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u16() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000000FF");
        let decoded = Parameter::decode(&ParameterType::Uint(16), &encoded).unwrap();

        let expected = Parameter::from_u16(255);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u32() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Uint(32), &encoded).unwrap();

        let expected = Parameter::from_u32(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u64() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Uint(64), &encoded).unwrap();

        let expected = Parameter::from_u64(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u128() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Uint(128), &encoded).unwrap();

        let expected = Parameter::from_u128(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_u256() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Uint(256), &encoded).unwrap();

        let expected = Parameter::from_u256(U256::from(511));
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_i8() {
        let encoded = hex!("000000000000000000000000000000000000000000000000000000000000007F");
        let decoded = Parameter::decode(&ParameterType::Int(8), &encoded).unwrap();

        let expected = Parameter::from_i8(127);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_i16() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000000FF");
        let decoded = Parameter::decode(&ParameterType::Int(16), &encoded).unwrap();

        let expected = Parameter::from_i16(255);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_i32() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Int(32), &encoded).unwrap();

        let expected = Parameter::from_i32(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    #[test]
    fn test_decode_i64() {
        let encoded = hex!("00000000000000000000000000000000000000000000000000000000000001FF");
        let decoded = Parameter::decode(&ParameterType::Int(64), &encoded).unwrap();
        let expected = Parameter::from_i64(511);
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }

    // TODO finish tests.
    #[test]
    #[ignore]
    fn decode_u128() {
        //let uint256 = Parameter::Uint128(U128::from_str("1555").unwrap());
        //let encoded = uint256.encode();
        //let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        //let expected = hex_val.to_vec();

        //let p = Parameter::decode(hex_val.to_vec(), ParameterType::Uint128(U128::zero()));
    }
}
