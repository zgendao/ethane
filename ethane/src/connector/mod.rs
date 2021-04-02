//! Allows connecting to an ethereum node

use crate::rpc::{sub::SubscriptionRequest, Rpc};
use crate::transport::{
    http::Http, websocket::WebSocket, websocket::WebSocketError, Credentials, Request, Subscribe,
    TransportError,
};

#[cfg(target_family = "unix")]
use crate::transport::uds::{Uds, UdsError};

use log::{debug, error, info, trace};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;
use thiserror::Error;

pub mod subscription;
use subscription::Subscription;

/// Used to to interact with ethereum nodes
///
/// This is basically a connection wrapper for sending requests or starting subscriptions.
pub struct Connector<T> {
    connection: T,
    id_pool: VecDeque<usize>,
}

impl Connector<Http> {
    /// Create a connector with a http connection. Does **not** allow to subscribe to node events.
    // This cannot return an error, but it does for convenience reasons. Should we change it?
    pub fn http(domain: &str, credentials: Option<Credentials>) -> Result<Self, ConnectorError> {
        info!("Creating connector over http");
        Ok(Connector {
            connection: Http::new(String::from(domain), credentials),
            id_pool: (0..1000).collect(),
        })
    }
}

impl Connector<WebSocket> {
    /// Create a connector with a websocket connection.
    pub fn websocket(
        domain: &str,
        credentials: Option<Credentials>,
    ) -> Result<Self, ConnectorError> {
        info!("Creating connector over websocket");
        Ok(Connector {
            connection: WebSocket::new(String::from(domain), credentials)?,
            id_pool: (0..1000).collect(),
        })
    }
}

#[cfg(target_family = "unix")]
impl Connector<Uds> {
    /// Create a connector using a unix domain socket.
    pub fn unix_domain_socket(path: &str) -> Result<Self, ConnectorError> {
        info!("Creating connector over unix domain socket");
        Ok(Connector {
            connection: Uds::new(String::from(path))?,
            id_pool: (0..1000).collect(),
        })
    }
}

impl<T: Subscribe + Request> Connector<T> {
    /// Starts a new subscription.
    /// Use one of these rpc generating [functions](crate::rpc::sub) to provide the subscription request.
    /// Returns a [subscription](Subscription) which you can poll for new items.
    pub fn subscribe<U: DeserializeOwned + Debug>(
        &mut self,
        sub_request: SubscriptionRequest<U>,
    ) -> Result<Subscription<U, T>, ConnectorError> {
        info!("Starting a new subscription");
        let mut connector = Connector {
            connection: self.connection.fork()?,
            id_pool: self.id_pool.clone(),
        };
        let subscription_id = connector.call(sub_request.rpc)?;
        Ok(Subscription {
            id: subscription_id,
            connector,
            result_type: PhantomData,
        })
    }
}

impl<T: Request> Connector<T> {
    /// Sends a request to an ethereum node. Use a function in one of these
    /// [functions](crate::rpc) to generate the request. Does **not** support the
    /// [subscription](crate::rpc::sub) namespace.
    pub fn call<U: DeserializeOwned + Debug>(
        &mut self,
        mut rpc: Rpc<U>,
    ) -> Result<U, ConnectorError> {
        let command_id = self.get_command_id()?;
        rpc.id = command_id;
        debug!("Calling rpc method: {:?}", &rpc);
        let response = self.connection.request(serde_json::to_string(&rpc)?)?;
        self.id_pool.push_back(command_id);
        deserialize_from_rpc(&response)
    }
}

impl<T> Connector<T> {
    fn get_command_id(&mut self) -> Result<usize, ConnectorError> {
        match self.id_pool.pop_front() {
            Some(inner) => {
                trace!("Using id {} for request", inner);
                Ok(inner)
            }
            None => Err(ConnectorError::NoTicketId),
        }
    }
}

fn deserialize_from_rpc<U: DeserializeOwned + Debug>(response: &str) -> Result<U, ConnectorError> {
    trace!("Deserializing response {}", response);
    match serde_json::from_str::<Response<U>>(response) {
        Ok(Response {
            result_or_error: inner,
            ..
        }) => match inner {
            RpcResult::Result(result) => Ok(result),
            RpcResult::Error(err) => Err(ConnectorError::from(err)),
        },
        Err(err) => Err(ConnectorError::from(err)),
    }
}

/// Used to deserialize errors returned from the ethereum node
#[derive(Deserialize, Debug, Error)]
#[error("{message}")]
pub struct JsonError {
    code: i32,
    message: String,
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    #[serde(flatten)]
    pub result_or_error: RpcResult<T>,
}

#[derive(Deserialize, Debug)]
enum RpcResult<T> {
    #[serde(rename = "result")]
    Result(T),
    #[serde(rename = "error")]
    Error(JsonError),
}

/// An error type collecting what can go wrong using a connector
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("Connector Websocket Init Error: {0}")]
    WsInit(#[from] WebSocketError),
    #[cfg(target_family = "unix")]
    #[error("Connector Unix Domain Socket Init Error: {0}")]
    UdsInit(#[from] UdsError),
    #[error("Connector Error: Maximum number of connections reached")]
    NoTicketId,
    #[error("Connector Transport Error: {0}")]
    Transport(#[from] TransportError),
    #[error("Node Response Error: {0:?}")]
    JsonRpc(#[from] JsonError),
    #[error("Connector De-/Serialization Error: {0}")]
    Serde(#[from] serde_json::Error),
}
