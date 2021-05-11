//! Implementation of http transport

use super::super::{ConnectionError, Credentials, Request};

use thiserror::Error;

/// Wraps a http client
pub struct Http {
    /// The domain where requests are sent
    address: String,
    credentials: Option<Credentials>,
    agent: ureq::Agent,
}

impl Http {
    pub fn new(address: &str, credentials: Option<Credentials>) -> Self {
        Self {
            address: address.to_owned(),
            credentials,
            agent: ureq::Agent::new(),
        }
    }

    fn prepare_json_request(&self) -> ureq::Request {
        let mut request = self.agent.request("POST", &self.address);
        if let Some(credentials) = &self.credentials {
            request = request.set("Authorization", &credentials.to_auth_string());
        }
        request = request.set("Content-Type", "application/json");
        request = request.set("Accept", "application/json");
        request
    }
}

impl Request for Http {
    fn request(&mut self, cmd: String) -> Result<String, ConnectionError> {
        let request = self.prepare_json_request();
        let response = request.send_string(&cmd).map_err(HttpError::from)?;
        response
            .into_string()
            .map(|resp| resp)
            .map_err(|err| HttpError::from(err).into())
    }
}

/// An error type collecting what can go wrong with http requests
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Http Address Error: {0}")]
    Uri(#[from] http::uri::InvalidUri),
    #[error("Http Response Parsing Error: {0}")]
    Conversion(#[from] std::io::Error),
    #[error("Http Send Request Error: {0}")]
    UreqError(#[from] ureq::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_request() {
        let address = "http://127.0.0.1";
        let credentials = Credentials::Basic(String::from("check!"));
        let client = Http::new(address, Some(credentials));
        let request = client.prepare_json_request();

        assert_eq!(request.header("Authorization").unwrap(), "Basic check!");
        assert_eq!(request.header("Content-Type").unwrap(), "application/json");
        assert_eq!(request.header("Accept").unwrap(), "application/json");
    }
}
