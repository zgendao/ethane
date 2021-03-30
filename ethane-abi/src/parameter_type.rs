use ethereum_types::{Address, U256, U128};
use serde_json::Value;

#[derive(Debug)]
pub enum ParameterType {
    Address(Address),
    Uint256(U256),
    Uint128(U128)
}

impl ParameterType {
    pub fn encode(&self) -> Value {
        match self {
            ParameterType::Address(address) => {
                Value::from(address.as_bytes())
            }
            ParameterType::Uint256(val) => {
                let mut padded: [u8; 32] = [0u8; 32];
                val.to_big_endian(&mut padded);
                Value::from(padded.to_vec())
            }
            ParameterType::Uint128(val) => {
                let mut padded: [u8; 16] = [0u8; 16];
                val.to_big_endian(&mut padded);
                let padded = pad_to_32_bytes(&padded);
                Value::from(padded.to_vec())
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


pub fn pad_to_32_bytes(value: &[u8]) -> [u8; 32] {
    assert!(value.len() <= 32, format!("Cannot pad to 32 bytes, input is too long ({} bytes)", value.len()));
    let mut padded = [0u8; 32];
    let diff = 32 - value.len();
    padded[diff..].copy_from_slice(value);
    padded
}

pub fn remove_padding_bytes(pad_length: usize,value: &[u8]) ->&[u8] {
    &value[pad_length..]
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use hex_literal::hex;
    use super::*;

    #[test]
    fn encode_address() {
        let address = ParameterType::Address(Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap());
        let encoded = address.encode();
        let hex_val = hex!("95eDA452256C1190947f9ba1fD19422f0120858a");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_u256() {
        let uint256 = ParameterType::Uint256(U256::from_str("1555").unwrap());
        let encoded = uint256.encode();
        let hex_val = hex!("0000000000000000000000000000000000000000000000000000000000001555");
        let expected = Value::from(hex_val.to_vec());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_u128() {
        let uint256 = ParameterType::Uint128(U128::from_str("1555").unwrap());
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
