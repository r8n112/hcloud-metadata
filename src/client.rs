//! The metadata service client.

use serde::de::DeserializeOwned;

use crate::error::{Error, Result};
use crate::transport::{Transport, UreqTransport};

/// Default base URL of the Hetzner Cloud instance metadata service.
pub const DEFAULT_BASE_URL: &str = "http://169.254.169.254";

/// Client for the Hetzner Cloud instance metadata service.
///
/// The generic parameter is the [`Transport`] used to perform requests, which
/// allows tests to run without network access.
pub struct MetadataClient<T = UreqTransport> {
    base_url: String,
    transport: T,
}

impl MetadataClient<UreqTransport> {
    /// Creates a client against [`DEFAULT_BASE_URL`] using the default transport.
    #[must_use]
    pub fn new() -> Self {
        Self::with_transport(DEFAULT_BASE_URL, UreqTransport::new())
    }
}

impl Default for MetadataClient<UreqTransport> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transport> MetadataClient<T> {
    /// Creates a client with an explicit base URL and transport.
    #[must_use]
    pub fn with_transport(base_url: impl Into<String>, transport: T) -> Self {
        let mut base_url = base_url.into();
        while base_url.ends_with('/') {
            base_url.pop();
        }
        Self {
            base_url,
            transport,
        }
    }

    /// Returns the configured base URL.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Fetches `path` and returns the raw response body.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotFound`] for HTTP 404, [`Error::Api`] for other
    /// non-success statuses, or [`Error::Transport`] if the request fails.
    pub fn get_text(&self, path: &str) -> Result<String> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let response = self.transport.get(&url)?;
        if response.status == 404 {
            return Err(Error::NotFound(path.to_owned()));
        }
        if !(200..300).contains(&response.status) {
            return Err(Error::Api {
                status: response.status,
                message: response.body.trim().chars().take(200).collect(),
            });
        }
        Ok(response.body)
    }

    /// Fetches `path` and deserialises the JSON body.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`get_text`](Self::get_text), plus
    /// [`Error::Decode`] if the body is not valid JSON.
    pub fn get_json<V: DeserializeOwned>(&self, path: &str) -> Result<V> {
        let body = self.get_text(path)?;
        serde_json::from_str(&body).map_err(|error| Error::Decode(error.to_string()))
    }
}
