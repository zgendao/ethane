use ethereum_types::H256;

#[derive(Clone)]
pub enum Parameter {
    Address(H256),
    Bool(H256),
    Int(H256, usize),
    Uint(H256, usize),
    String(Vec<u8>),
    Bytes(Vec<u8>),
    Array(Vec<Parameter>),
    Tuple(Vec<Parameter>),
}
