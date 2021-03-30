//! Possible transports which are supported by the [Connector](crate::Connector)

use thiserror::Error;
pub mod http;
#[cfg(target_family = "unix")]
pub mod uds;
pub mod websocket;

/// Implemented by transports which offer request functionality
pub trait Request {
    fn request(&mut self, cmd: String) -> Result<String, TransportError>;
}

/// Trait for transports which offer subscriptions
pub trait Subscribe {
    fn read_next(&mut self) -> Result<String, TransportError>;
    fn fork(&self) -> Result<Self, TransportError>
    where
        Self: Sized;
}

/// Credentials can be used for authentication
///
/// Use this when creating a [connector](crate::Connector). Supports Basic and Bearer authentication.
/// So you can easily add HTTP Basic or JWT authentication. This will add an authorization header
/// to your requests and works for [websockets](crate::Connector::websocket) and
/// [http](crate::Connector::http).
#[derive(Debug, Clone)]
pub enum Credentials {
    Bearer(String),
    Basic(String),
}

impl Credentials {
    pub fn to_auth_string(&self) -> String {
        match self {
            Self::Bearer(token) => String::from("Bearer ") + &token,
            Self::Basic(token) => String::from("Basic ") + &token,
        }
    }
}

/// Wraps the different transport errors that can happen
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum TransportError {
    #[error("{0}")]
    WebSocketError(#[from] websocket::WebSocketError),
    #[error("{0}")]
    HttpError(#[from] http::HttpError),
    #[cfg(target_family = "unix")]
    #[error("{0}")]
    UdsError(#[from] uds::UdsError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials() {
        let auth_string = "my_awesome_auth_string";
        assert_eq!(
            Credentials::Bearer(auth_string.to_string()).to_auth_string(),
            String::from("Bearer ") + auth_string
        );
        assert_eq!(
            Credentials::Basic(auth_string.to_string()).to_auth_string(),
            String::from("Basic ") + auth_string
        );
    }
}
