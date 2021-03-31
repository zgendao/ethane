use crate::{Parameter, ParameterType};
use ethereum_types::Address;
use std::convert::TryInto;

impl Parameter {
    pub fn decode(param_type: ParameterType, raw_bytes: &[u8]) -> (Self,usize) {
        // TODO validate raw_bytes length
        match param_type {
            ParameterType::Address => (Self::Address(Address::from_slice(remove_left_padding_bytes(12,&raw_bytes[..32]))),32),
            ParameterType::Bool => (Self::Bool(raw_bytes[31] == 1),32),
            ParameterType::Int(length) => {
                let cleaned = remove_left_padding_bytes(32 - (length / 8),&raw_bytes[..32]);
                match length {
                    8 => (Self::Uint8(cleaned.try_into().expect("input with incorrect length")),32),
                    16 => (Self::Uint16(cleaned.try_into().expect("input with incorrect length")),32),
                    32 => (Self::Uint32(cleaned.try_into().expect("input with incorrect length")),32),
                    64 => (Self::Uint64(cleaned.try_into().expect("input with incorrect length")),32),
                    128 => (Self::Uint128(cleaned.try_into().expect("input with incorrect length")),32),
                    256 => (Self::Uint256(cleaned.try_into().expect("input with incorrect length")),32),
                    _ => unimplemented!()
                }
            }
            ParameterType::Uint(length) => {
                let cleaned = remove_left_padding_bytes(32 - (length / 8),&raw_bytes[..32]);
                match length {
                    8 => (Self::Int8(cleaned.try_into().expect("input with incorrect length")),32),
                    16 => (Self::Int16(cleaned.try_into().expect("input with incorrect length")),32),
                    32 => (Self::Int32(cleaned.try_into().expect("input with incorrect length")),32),
                    64 => (Self::Int64(cleaned.try_into().expect("input with incorrect length")),32),
                    128 => (Self::Int128(cleaned.try_into().expect("input with incorrect length")),32),
                    256 => (Self::Int256(cleaned.try_into().expect("input with incorrect length")),32),
                    _ => unimplemented!()
                }
            }
            _ => unimplemented!(),
            // ParameterType::Bytes => {}
            // ParameterType::FixedBytes(_) => {}

            // ParameterType::String => {}
        }
    }
}

pub fn remove_left_padding_bytes(pad_length: usize, value: &[u8]) -> &[u8] {
    &value[pad_length..]
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_decode_address() {
        let encoded_address =  hex!("00000000000000000000000095eda452256c1190947f9ba1fd19422f0120858a1234");
        let decoded = Parameter::decode(ParameterType::Address, &encoded_address);

        let expected = Parameter::Address(
            Address::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap(),
        );
        assert_eq!(decoded.0, expected);
        assert_eq!(decoded.1, 32);
    }
}
