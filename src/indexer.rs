use reqwest::header::HeaderMap;

use crate::algod::models::{ NodeStatus};
use crate::{Error};

const AUTH_HEADER: &str = "X-Indexer-API-Token";

/// Client for interacting with the Algorand protocol daemon
pub struct IndexerClient {
    url: String,
    token: String,
    headers: HeaderMap,
}

impl IndexerClient {
    pub fn new(address: &str, token: &str) -> IndexerClient {
        IndexerClient::new_with_headers(address, token, HeaderMap::new())
    }

    pub fn new_with_headers(address: &str, token: &str, headers: HeaderMap) -> IndexerClient {
        IndexerClient {
            url: address.to_string(),
            token: token.to_string(),
            headers,
        }
    }
    /// Returns Ok if healthy
    pub fn health(&self) -> Result<(), Error> {
        let _ = reqwest::Client::new()
            .get(&format!("{}/health", self.url))
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?;
        Ok(())
    }
    /// Gets the current node status
    pub fn status(&self) -> Result<NodeStatus, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/v2/status", self.url))
            .header(AUTH_HEADER, &self.token)
            .headers(self.headers.clone())
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response)
    }
}

pub mod requests {
    use reqwest::Method;
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    pub trait APIV1Request: Serialize {
        type Response: DeserializeOwned;
        const PATH: &'static str;
        const METHOD: Method;
    }
}
