use ethereum_types::U256;

pub enum Parameter {
    Address(U256),
    Bool(U256),
    Int(U256),
    Uint(U256),
    String(Vec<u8>),
    Bytes(Vec<u8>),
    Array(Vec<Parameter>),
    Tuple(Vec<Parameter>),
}
