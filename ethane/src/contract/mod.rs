#[cfg(feature = "blocking")]
mod blocking;
#[cfg(feature = "blocking")]
pub use blocking::Caller;
#[cfg(feature = "non-blocking")]
mod non_blocking;
#[cfg(feature = "non-blocking")]
pub use non_blocking::Caller as AsyncCaller;

use crate::types::{Address, H256};
use ethane_abi::Parameter;

pub struct CallOpts {
    pub force_call_type: Option<CallType>,
    pub from: Option<Address>,
}

pub enum CallType {
    Transaction,
    Call,
}

pub enum CallResult {
    Transaction(H256),
    Call(Vec<Parameter>),
}
