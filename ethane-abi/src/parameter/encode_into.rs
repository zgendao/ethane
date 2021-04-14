use super::tmp::Parameter;
use super::utils::left_pad_to_32_bytes;

use std::ops::Range;

pub fn encode_into(hash: &mut Vec<u8>, parameters: Vec<Parameter>) {
    let mut hash_len = hash.len();
    let mut dynamic_type_map = Vec::<(usize, Range<usize>)>::with_capacity(parameters.len());
    for (i, param) in parameters.iter().enumerate() {
        if param.is_dynamic() {
            // save range where we will insert the data pointer since
            // we don't know (YET) where exactly the dynamic data will
            // start
            dynamic_type_map.push((i, hash_len..hash_len + 32));
            // append a 32 byte zero slice as a placeholder for our
            // future dynamic data pointer
            hash.extend_from_slice(&[0u8; 32]);
        } else {
            match param {
                // in case of a static tuple,
                // recursively encode the internal data
                Parameter::Tuple(data) => encode_into(hash, data.to_vec()),
                _ => hash.extend_from_slice(&param.static_encode()),
            }
        }
        // update hash position (length)
        hash_len = hash.len();
    }

    for (index, range) in dynamic_type_map.into_iter() {
        // fill in the pointer offset to the dynamic data
        // the offset is measured from the 4th byte so the
        // Keccak method ID doesn't count
        hash[range].copy_from_slice(&left_pad_to_32_bytes(&(hash_len - 4).to_be_bytes()));
        match &parameters[index] {
            Parameter::Array(data) | Parameter::Tuple(data) => {
                // encode the length of the underlying dynamic data
                hash.extend_from_slice(&left_pad_to_32_bytes(&data.len().to_be_bytes()));
                encode_into(hash, data.to_vec());
            }
            _ => hash.extend_from_slice(&parameters[index].static_encode()),
        }
        hash_len = hash.len();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ethereum_types::U256;

    #[test]
    #[rustfmt::skip]
    fn first_contract_abi() {
        let mut hash = vec![0xaa, 0xbb, 0xcc, 0xdd];
        encode_into(
            &mut hash,
            vec![Parameter::from(69u32), Parameter::from(true)],
        );
        assert_eq!(
            hash,
            vec![
                0xaa, 0xbb, 0xcc, 0xdd, // keccak signature
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0x45, // u32
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0x01, // bool
            ]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn second_contract_abi() {
        let mut hash = vec![0x10, 0x11, 0x12, 0x13];
        encode_into(
            &mut hash,
            vec![Parameter::FixedArray(vec![
                Parameter::new_fixed_bytes(&"abc".as_bytes()),
                Parameter::new_fixed_bytes(&"def".as_bytes()),
            ])],
        );
        assert_eq!(
            hash,
            vec![
                0x10, 0x11, 0x12, 0x13, // keccak signature
                0x61, 0x62, 0x63, 0, 0, 0, 0, 0, // "abc"
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0x64, 0x65, 0x66, 0, 0, 0, 0, 0, // "def"
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn third_contract_abi() {
        let mut hash = vec![0xff, 0xee, 0x00, 0x07];
        encode_into(&mut hash, vec![
            Parameter::new_bytes(&"dave".as_bytes()),
            Parameter::from(true),
            Parameter::Array(vec![
                Parameter::from(U256::from_dec_str("1").unwrap()),
                Parameter::from(U256::from_dec_str("2").unwrap()),
                Parameter::from(U256::from_dec_str("3").unwrap()),
            ])
        ]);
        assert_eq!(hash, vec![
            0xff, 0xee, 0x00, 0x07, // signature
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x60, // bytes data will start at a 96 byte offset from signature
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x01, // bool true
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0xa0, // array data will start at a 160 byte offset from signature
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x04, // "dave" consists of 4 bytes
            0x64, 0x61, 0x76, 0x65, 0, 0, 0, 0, // "dave" as bytes
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, // byte array is padded right
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x03, // uint array has 3 elements
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x01, // first element
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x02, // second element
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0x03, // third element
        ]);
    }
}
