use crate::connection::{ConnectionError, Credentials};

use reqwest::header::HeaderMap;
use reqwest::Client;

/// Wraps a blocking http client
#[derive(Clone)]
pub struct Http {
    /// The domain where requests are sent
    address: String,
    credentials: Option<Credentials>,
    client: Client,
}

impl Http {
    pub fn new(address: &str, credentials: Option<Credentials>) -> Self {
        Self {
            address: address.to_owned(),
            credentials,
            client: Client::new(),
        }
    }

    fn json_request_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(credentials) = &self.credentials {
            headers.insert(
                "Authorization",
                credentials.to_auth_string().parse().unwrap(),
            );
        }
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
        headers
    }

    pub async fn request(&self, cmd: String) -> Result<String, ConnectionError> {
        self.client
            .post(&self.address)
            .headers(self.json_request_headers())
            .body(cmd)
            .send()
            .await
            .map_err(|e| ConnectionError::HttpError(e.to_string()))?
            .text()
            .await
            .map_err(|e| ConnectionError::HttpError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_request() {
        let address = "http://127.0.0.1";
        let credentials = Credentials::Basic(String::from("check!"));
        let client = Http::new(address, Some(credentials));
        let headers = client.json_request_headers();

        assert_eq!(headers.get("Authorization").unwrap(), "Basic check!");
        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(headers.get("Accept").unwrap(), "application/json");
    }
}
