//! HTTP transport abstraction.
//!
//! The [`Transport`] trait keeps the client testable: production uses
//! [`UreqTransport`] while tests inject an in-memory mock and never touch the
//! network.

use crate::error::{Error, Result};

/// A minimal HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response body as UTF-8 text.
    pub body: String,
}

/// Executes HTTP `GET` requests on behalf of the client.
pub trait Transport {
    /// Fetches `url`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Transport`] if no response could be obtained, or
    /// [`Error::Decode`] if the body could not be read as text.
    fn get(&self, url: &str) -> Result<HttpResponse>;
}

/// Blocking HTTP transport backed by [`ureq`].
#[derive(Debug)]
pub struct UreqTransport {
    agent: ureq::Agent,
}

impl UreqTransport {
    /// Creates a transport with a default agent.
    #[must_use]
    pub fn new() -> Self {
        Self {
            agent: ureq::AgentBuilder::new().build(),
        }
    }
}

impl Default for UreqTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for UreqTransport {
    fn get(&self, url: &str) -> Result<HttpResponse> {
        match self.agent.get(url).call() {
            Ok(response) => read_response(response.status(), response),
            Err(ureq::Error::Status(status, response)) => read_response(status, response),
            Err(ureq::Error::Transport(error)) => Err(Error::Transport(error.to_string())),
        }
    }
}

fn read_response(status: u16, response: ureq::Response) -> Result<HttpResponse> {
    let body = response
        .into_string()
        .map_err(|error| Error::Decode(error.to_string()))?;
    Ok(HttpResponse { status, body })
}
