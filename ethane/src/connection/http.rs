//! Implementation of http transport

use super::Credentials;
use crate::transport::{Request, TransportError};
use log::{debug, trace};
use thiserror::Error;
use ureq::{Agent, Error as UreqError, Request as UreqRequest};

/// Wraps a http client
pub struct Http {
    /// The domain where requests are sent
    pub address: String,
    pub(crate) credentials: Option<Credentials>,
    agent: Agent,
}
/*
impl Http {
    pub(crate) fn new(address: String, credentials: Option<Credentials>) -> Self {
        debug!("Binding http client to {}", address);
        Http {
            agent: Agent::new(),
            address,
            credentials,
        }
    }

    fn prepare_json_request(&self) -> UreqRequest {
        let domain = self.address.clone();
        let mut request = self.agent.request("POST", &domain);
        if let Some(ref credentials) = self.credentials {
            request = request.set("Authorization", &credentials.to_auth_string());
        }
        request = request.set("Content-Type", "application/json");
        request = request.set("Accept", "application/json");
        request
    }
}

impl Request for Http {
    fn request(&mut self, cmd: String) -> Result<String, TransportError> {
        let request = self.prepare_json_request();
        trace!("Sending request {:?} with body {}", &request, &cmd);
        let response = request.send_string(&cmd).map_err(HttpError::from)?;
        response
            .into_string()
            .map(|resp| {
                trace!("Received http response: {}", &resp);
                resp
            })
            .map_err(|err| HttpError::from(err).into())
    }
}
*/
/// An error type collecting what can go wrong with http requests
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Http Address Error: {0}")]
    Uri(#[from] http::uri::InvalidUri),
    #[error("Http Response Parsing Error: {0}")]
    Conversion(#[from] std::io::Error),
    #[error("Http Send Request Error: {0}")]
    UreqError(#[from] UreqError),
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_prepare_request() {
        let address = String::from("http://127.0.0.1");
        let credentials = Credentials::Basic(String::from("check!"));
        let client = Http::new(address, Some(credentials));
        let request = client.prepare_json_request();

        assert_eq!(request.header("Authorization").unwrap(), "Basic check!");
        assert_eq!(request.header("Content-Type").unwrap(), "application/json");
        assert_eq!(request.header("Accept").unwrap(), "application/json");
    }
}
*/
