use super::Parameter;
impl Parameter {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Parameter::Address(address) => address.as_bytes().to_vec(),
            Parameter::Uint8(val) | Parameter::Int8(val) => left_pad_to_32_bytes(val).to_vec(),
            Parameter::Uint16(val) | Parameter::Int16(val) => left_pad_to_32_bytes(val).to_vec(),
            Parameter::Uint32(val) | Parameter::Int32(val) => left_pad_to_32_bytes(val).to_vec(),
            Parameter::Uint64(val) | Parameter::Int64(val) => {
                let mut padded: [u8; 8] = [0u8; 8];
                val.to_big_endian(&mut padded);
                left_pad_to_32_bytes(&padded).to_vec()
            }
            Parameter::Uint128(val) | Parameter::Int128(val) => {
                let mut padded: [u8; 16] = [0u8; 16];
                val.to_big_endian(&mut padded);
                left_pad_to_32_bytes(&padded).to_vec()
            }
            Parameter::Uint256(val) | Parameter::Int256(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                val.to_big_endian(&mut padded);
                padded.to_vec()
            }
            Parameter::Bool(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                if *val {
                    padded[31] = 1;
                }
                padded.to_vec()
            }
            Parameter::FixedBytes(val) => right_pad_to_32_bytes(val).to_vec(),
            Parameter::Bytes(bytes) => {
                let mut res: Vec<u8> = vec![];
                // number of bytes is encoded as a uint256
                let length: [u8; 32] = left_pad_to_32_bytes(&bytes.len().to_be_bytes());
                res.extend_from_slice(&length);
                res.extend_from_slice(right_pad_bytes(&bytes).as_slice());
                res
            }
            Parameter::String(val) => Parameter::Bytes(Vec::from(val.as_bytes())).encode(),
        }
    }
}

pub fn left_pad_to_32_bytes(value: &[u8]) -> [u8; 32] {
    assert!(
        value.len() <= 32,
        format!(
            "Cannot pad to 32 bytes, input is too long ({} bytes)",
            value.len()
        )
    );
    let mut padded = [0u8; 32];
    let diff = 32 - value.len();
    padded[diff..].copy_from_slice(value);
    padded
}

pub fn right_pad_to_32_bytes(value: &[u8]) -> [u8; 32] {
    assert!(
        value.len() <= 32,
        format!(
            "Cannot pad to 32 bytes, input is too long ({} bytes)",
            value.len()
        )
    );
    let mut padded = [0u8; 32];
    let diff = 32 - value.len();
    padded[..(32 - diff)].copy_from_slice(value);
    padded
}

pub fn right_pad_bytes(value: &[u8]) -> Vec<u8> {
    let mut length = value.len();
    while length % 32 != 0 {
        length += 1;
    }
    let mut padded: Vec<u8> = vec![0; length];
    let diff = length - value.len();
    padded[..(length - diff)].copy_from_slice(value);
    padded
}

#[cfg(test)]
mod test {
    use super::super::*;
    use hex_literal::hex;
    use std::str::FromStr;

    #[test]
    fn test_encode_address() {
        let address = Parameter::Address(
            Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
        );
        let encoded = address.encode();
        let hex_val = hex!("95eDA452256C1190947f9ba1fD19422f0120858a");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u8() {
        let uint8 = Parameter::from_u8(11);
        let encoded = uint8.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000000B");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u16() {
        let uint16 = Parameter::from_u16(123);
        let encoded = uint16.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000007B");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u32() {
        let uint32 = Parameter::from_u16(65535);
        let encoded = uint32.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000FFFF");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u64() {
        let uint64 = Parameter::from_u64(18_446_744_073_709_551_615);
        let encoded = uint64.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000FFFFFFFFFFFFFFFF");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u128() {
        let uint256 = Parameter::Uint128(U128::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_u256() {
        let uint256 = Parameter::Uint256(U256::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i8() {
        let int8 = Parameter::from_i8(11);
        let encoded = int8.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000000B");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i16() {
        let int16 = Parameter::from_i16(123);
        let encoded = int16.encode();
        let hex_val = hex!("000000000000000000000000000000000000000000000000000000000000007B");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i32() {
        let int32 = Parameter::from_i16(32767);
        let encoded = int32.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000007FFF");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i64() {
        let int64 = Parameter::from_i64(9_223_372_036_854_775_807);
        let encoded = int64.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000007FFFFFFFFFFFFFFF");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i128() {
        let int256 = Parameter::Uint128(U128::from_str("1555").unwrap());
        let encoded = int256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_i256() {
        let int256 = Parameter::Uint256(U256::from_str("1555").unwrap());
        let encoded = int256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_encode_string() {
        let str = Parameter::String(String::from("AAAA"));
        let encoded = str.encode();
        println!("{:?}", encoded);

        let hex_val = hex!("00000000000000000000000000000000000000000000000000000000000000044141414100000000000000000000000000000000000000000000000000000000");
        let expected = hex_val.to_vec();
        assert_eq!(encoded, expected);
    }
}
