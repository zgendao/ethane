use super::Rpc;
use crate::types::{TxPoolContent, TxPoolInspect, TxPoolStatus};

pub fn txpool_status() -> Rpc<TxPoolStatus> {
    Rpc::new("txpool_status")
}

pub fn txpool_content() -> Rpc<TxPoolContent> {
    Rpc::new("txpool_content")
}

pub fn txpool_inspect() -> Rpc<TxPoolInspect> {
    Rpc::new("txpool_inspect")
}
