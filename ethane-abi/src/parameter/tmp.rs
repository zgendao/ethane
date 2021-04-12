use ethereum_types::H256;

pub enum Parameter {
    Address(H256),
    Bool(H256),
    Int(H256),
    Uint(H256),
    String(Vec<u8>),
    Bytes(Vec<u8>),
    Array(Vec<Parameter>),
    Tuple(Vec<Parameter>),
}
