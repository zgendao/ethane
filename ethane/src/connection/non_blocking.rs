use super::transport::http::AsyncHttp;
use super::ConnectionError;
use crate::rpc::{Rpc, RpcResponse};

use serde::de::DeserializeOwned;

pub struct Connection {
    transport: AsyncHttp,
    id_pool: std::collections::VecDeque<usize>,
}

impl Connection {
    pub fn new(address: &str) -> Self {
        Self {
            transport: AsyncHttp::new(&address, None),
            id_pool: (0..1000).collect(),
        }
    }

    pub async fn call<U>(&mut self, mut rpc: Rpc<U>) -> Result<U, ConnectionError>
    where
        U: DeserializeOwned + std::fmt::Debug,
    {
        if let Some(id) = self.id_pool.pop_front() {
            rpc.id = id;
            self.id_pool.push_back(id);
            let result_data = self
                .transport
                .request(
                    serde_json::to_string(&rpc)
                        .map_err(|e| ConnectionError::Serde(e.to_string()))?,
                )
                .await?;
            let result = serde_json::from_str::<RpcResponse<U>>(&result_data)
                .map_err(|e| ConnectionError::Serde(e.to_string()))?;
            Ok(result.result)
        } else {
            Err(ConnectionError::NoTicketId)
        }
    }
}
