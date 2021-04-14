use super::tmp::Parameter;
use super::utils::left_pad_to_32_bytes;

use std::collections::HashMap;
use std::ops::Range;

pub fn encode_into(hash: &mut Vec<u8>, parameters: Vec<Parameter>) {
    let mut hash_len = hash.len();
    let mut dynamic_type_map = HashMap::<usize, Range<usize>>::with_capacity(parameters.len());
    for (i, param) in parameters.iter().enumerate() {
        if param.is_dynamic() {
            // save range where we will insert the data pointer since
            // we don't know (YET) where exactly the dynamic data will
            // start
            dynamic_type_map.insert(i, hash_len..hash_len + 32);
            // append a 32 byte zero slice as a placeholder for our
            // future dynamic data pointer
            hash.extend_from_slice(&[0u8; 32]);
            // update hash position (length)
            hash_len = hash.len();
        } else {
            match param {
                Parameter::Array(data) | Parameter::Tuple(data) => encode_into(hash, data.to_vec()),
                _ => hash.extend_from_slice(&param.encode()),
            }
        }
    }

    for (index, range) in dynamic_type_map {
        match &parameters[index] {
            Parameter::Array(data) | Parameter::Tuple(data) => encode_into(hash, data.to_vec()),
            _ => {
                // fill in the pointer offset to the dynamic data
                hash[range].copy_from_slice(&left_pad_to_32_bytes(&hash_len.to_be_bytes()));
                // append the encoded data
                hash.extend_from_slice(&parameters[index].encode());
                hash_len = hash.len();
            }
        }
    }
}

#[cfg(test)]
mod test {
}
