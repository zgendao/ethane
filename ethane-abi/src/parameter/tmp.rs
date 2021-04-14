use ethereum_types::H256;

#[derive(Clone)]
pub enum Parameter {
    // TODO make this public only on crate level (otherwise ppl may generate String/Bytes variants without encoding.
    Address(H256),
    Bool(H256),
    Int(H256, usize),
    Uint(H256, usize),
    String(Vec<u8>),
    Bytes(Vec<u8>),
    Array(Vec<Parameter>),
    Tuple(Vec<Parameter>),
}

impl Parameter {
    /// Returns the encoded data length in bytes.
    #[inline]
    #[rustfmt::skip]
    pub fn data_len(&self) -> usize {
        match self {
            // add 32 bytes to types encoding the lenght
            Self::String(data) | Self::Bytes(data) => 32 + data.len(),
            Self::Array(data) => if data.is_empty() { 0 } else { 32 + data.len() * data[1].data_len() },
            Self::Tuple(data) => data.iter().fold(32, |acc, d| acc + d.data_len()),
            _ => 32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parameter_data_length() {
        assert_eq!(Parameter::Address(H256::zero()).data_len(), 32);
        assert_eq!(Parameter::Bool(H256::zero()).data_len(), 32);
        assert_eq!(Parameter::Int(H256::zero(), 16).data_len(), 32);
        assert_eq!(Parameter::Uint(H256::zero(), 16).data_len(), 32);
        assert_eq!(Parameter::String(vec![14; 64]).data_len(), 96);
        assert_eq!(Parameter::Bytes(vec![122; 128]).data_len(), 160);
        assert_eq!(
            Parameter::Array(vec![Parameter::String(vec![22; 32]); 5]),
            5 * 64 + 32
        );
    }
}
