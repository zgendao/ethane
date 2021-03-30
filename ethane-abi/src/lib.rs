use ethereum_types::{Address, U256};

pub enum ParameterType {
    Address(Address),
    Uint256(U256),
}

//impl ParameterType {
//    pub fn encode(&self) -> serde_json {
//    }
//
//    pub fn decode(raw: &serde_json::Value) -> Self {
//    }
//}
//

pub fn pad_to_32_bytes(value: &[u8]) -> [u8; 32] {
    assert!(value.len() < 32, format!("Cannot pad to 32 bytes, input is too long ({} bytes)", value.len()));
    let mut padded = [0u8; 32];
    let diff = 32 - value.len();
    padded[diff..].copy_from_slice(value);
    padded
}

#[cfg(test)]
mod test {

}
