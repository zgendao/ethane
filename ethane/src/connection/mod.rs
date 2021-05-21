#[cfg(feature = "blocking")]
mod blocking;
mod credentials;
#[cfg(feature = "non-blocking")]
mod non_blocking;
mod subscription;
mod transport;

#[cfg(feature = "blocking")]
pub use blocking::Connection;
pub use credentials::Credentials;
#[cfg(feature = "non-blocking")]
pub use non_blocking::Connection as AsyncConnection;
#[cfg(feature = "blocking")]
pub use subscription::Subscription;
#[cfg(feature = "blocking")]
pub use transport::http::Http;
#[cfg(target_family = "unix")]
pub use transport::uds::Uds;
pub use transport::websocket::WebSocket;

pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, ConnectionError>;
}

pub trait Subscribe {
    fn read_next(&mut self) -> Result<String, ConnectionError>;
    fn fork(&self) -> Result<Self, ConnectionError>
    where
        Self: Sized;
}

/// Wraps the different transport errors that may occur.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum ConnectionError {
    WebSocketError(String),
    HttpError(String),
    UdsError(String),
    JsonRpc(String),
    Serde(String),
    SubscriptionError(String),
    NoTicketId,
}
