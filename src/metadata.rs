//! Typed access to the instance metadata document.

use serde::{Deserialize, Serialize};

use crate::client::MetadataClient;
use crate::error::Result;
use crate::transport::Transport;

/// Path of the full metadata document.
pub const METADATA_PATH: &str = "/hetzner/v1/metadata";

/// Path of the cloud-init user data.
pub const USER_DATA_PATH: &str = "/hetzner/v1/userdata";

/// The instance metadata document.
///
/// Unknown or not-yet-modelled fields are tolerated; `private_networks` is kept
/// as raw JSON until a typed model is added (see `BACKLOG.md`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    /// Instance ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    /// Instance hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Availability zone, for example `fsn1-dc14`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// Region, for example `fsn1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Public IPv4 address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_ipv4: Option<String>,
    /// Public IPv6 address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_ipv6: Option<String>,
    /// SSH public keys registered for the instance.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<String>,
    /// Raw `private-networks` value.
    #[serde(default)]
    pub private_networks: serde_json::Value,
}

/// Fetches the full metadata document.
///
/// # Errors
///
/// Propagates errors from the underlying client: [`crate::Error::Api`] for a
/// non-success status, [`crate::Error::Decode`] for malformed JSON,
/// [`crate::Error::NotFound`] if the document is unavailable, and
/// [`crate::Error::Transport`] for request failures.
pub fn get<T: Transport>(client: &MetadataClient<T>) -> Result<Metadata> {
    client.get_json(METADATA_PATH)
}

/// Fetches a single metadata field as raw text.
///
/// `key` is the metadata key as exposed by the service, for example
/// `instance-id`, `hostname`, `availability-zone`, `public-ipv4`, or
/// `public-keys`.
///
/// # Errors
///
/// Propagates errors from the underlying client; see [`get`].
pub fn field<T: Transport>(client: &MetadataClient<T>, key: &str) -> Result<String> {
    client.get_text(&format!("{METADATA_PATH}/{key}"))
}

/// Fetches the cloud-init user data.
///
/// # Errors
///
/// Propagates errors from the underlying client; see [`get`].
pub fn user_data<T: Transport>(client: &MetadataClient<T>) -> Result<String> {
    client.get_text(USER_DATA_PATH)
}
