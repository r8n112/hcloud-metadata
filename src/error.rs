//! Error types for `hcloud-metadata`.

use thiserror::Error;

/// Errors produced by the library.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The HTTP transport failed before a response was received.
    #[error("transport error: {0}")]
    Transport(String),

    /// The metadata service returned a non-success status.
    #[error("metadata service returned HTTP {status}: {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Short message extracted from the response.
        message: String,
    },

    /// A response body could not be decoded.
    #[error("failed to decode response: {0}")]
    Decode(String),

    /// The requested metadata key does not exist.
    #[error("metadata key not found: {0}")]
    NotFound(String),
}

/// Convenience result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
