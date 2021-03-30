use ethereum_types::{Address, U128, U256, U64};
use serde_json::Value;

#[derive(Debug)]
pub enum Parameter {
    Address(Address),
    Uint8([u8; 1]),
    Uint16([u8; 2]),
    Uint32([u8; 4]),
    Uint64(U64),
    Uint128(U128),
    Uint256(U256),
    Int8([u8; 1]),
    Int16([u8; 2]),
    Int32([u8; 4]),
    Int64(U64),
    Int128(U128),
    Int256(U256),
    Bool(bool),
    // Vector of bytes with known size.
    FixedBytes(Vec<u8>),
    // dynamic sized byte sequence
    Bytes(Vec<u8>),
    // dynamic sized unicode string assumed to be UTF-8 encoded.
    String(String),
}

impl Parameter {
    pub fn encode(&self) -> Value {
        match self {
            Parameter::Address(address) => Value::from(address.as_bytes()),
            Parameter::Uint8(val) | Parameter::Int8(val) => {
                Value::from(left_pad_to_32_bytes(val).to_vec())
            }
            Parameter::Uint16(val) | Parameter::Int16(val) => {
                Value::from(left_pad_to_32_bytes(val).to_vec())
            }
            Parameter::Uint32(val) | Parameter::Int32(val) => {
                Value::from(left_pad_to_32_bytes(val).to_vec())
            }
            Parameter::Uint64(val) | Parameter::Int64(val) => {
                let mut padded: [u8; 8] = [0u8; 8];
                val.to_big_endian(&mut padded);
                let padded = left_pad_to_32_bytes(&padded);
                Value::from(padded.to_vec())
            }
            Parameter::Uint128(val) | Parameter::Int128(val) => {
                let mut padded: [u8; 16] = [0u8; 16];
                val.to_big_endian(&mut padded);
                let padded = left_pad_to_32_bytes(&padded);
                Value::from(padded.to_vec())
            }
            Parameter::Uint256(val) | Parameter::Int256(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                val.to_big_endian(&mut padded);
                Value::from(padded.to_vec())
            }
            Parameter::Bool(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                if *val {
                    padded[31] = 1;
                }
                Value::from(padded.to_vec())
            }
            Parameter::FixedBytes(val) => {
                Value::from(right_pad_to_32_bytes(val).to_vec())
            }
            Parameter::Bytes(bytes) => {
                let mut res: Vec<u8> = vec![];
                // number of bytes is encoded as a uint256
                let length: [u8; 32] = left_pad_to_32_bytes(&bytes.len().to_be_bytes());
                res.extend_from_slice(&length);
                res.extend_from_slice(right_pad_bytes(&bytes).as_slice());

                Value::from(res)
            }
            Parameter::String(val) => {
                Value::from(val.as_bytes())
            }
        }
    }

    // pub fn decode(raw: Vec<u8>,p: ParameterType) -> Self {
    //     match p {
    //         ParameterType::Address(_) => ParameterType::Address(Address::from_slice(remove_padding_bytes(12,&raw))),
    //         ParameterType::Uint256(_) => ParameterType::Uint256(U256::from_big_endian(remove_padding_bytes(0,&raw))),
    //         ParameterType::Uint128(_) => ParameterType::Uint128(U128::from_big_endian(remove_padding_bytes(16,&raw))),
    //     }
    // }
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

fn right_pad_bytes(value: &[u8]) -> Vec<u8> {
    let mut length = value.len();
    while length % 32 != 0 {
        length += 1;
    }
    let mut padded: Vec<u8> = vec![0;length];
    let diff = length - value.len();
    padded[..(length - diff)].copy_from_slice(value);
    padded
}

pub fn remove_padding_bytes(pad_length: usize, value: &[u8]) -> &[u8] {
    &value[pad_length..]
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use hex_literal::hex;

    use super::*;

    #[test]
    fn encode_address() {
        let address = Parameter::Address(
            Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
        );
        let encoded = address.encode();
        let hex_val = hex!("95eDA452256C1190947f9ba1fD19422f0120858a");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_u256() {
        let uint256 = Parameter::Uint256(U256::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_u128() {
        let uint256 = Parameter::Uint128(U128::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }
    //
    // #[test]
    // fn decode_u128() {
    //     let uint256 = ParameterType::Uint128(U128::from_str("1555").unwrap());
    //     let encoded = uint256.encode();
    //     let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
    //     let expected = Value::from(hex_val.to_vec());
    //
    //     let p = ParameterType::decode(hex_val.to_vec(),ParameterType::Uint128(U128::zero()));
    //     println!("{:?}",p)
    // }
}
