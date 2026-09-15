//! `hcloud-metadata` — a low-level client and CLI for the Hetzner Cloud instance
//! metadata service (IMDS) at `169.254.169.254`.
//!
//! The crate is split into a small, testable library and a thin binary:
//!
//! - [`MetadataClient`] performs requests over a pluggable [`Transport`], so
//!   tests run entirely in memory.
//! - [`metadata`] contains typed access to the metadata document.
//! - the `hcloud-metadata` binary is a thin wrapper around the library.
//!
//! # Examples
//!
//! ```no_run
//! use hcloud_metadata::{metadata, MetadataClient};
//!
//! let client = MetadataClient::new();
//! let info = metadata::get(&client)?;
//! println!("{:?}", info.hostname);
//! # Ok::<(), hcloud_metadata::Error>(())
//! ```

#![deny(missing_docs)]

mod client;
mod error;
pub mod metadata;
mod transport;

pub use client::{MetadataClient, DEFAULT_BASE_URL};
pub use error::{Error, Result};
pub use transport::{HttpResponse, Transport, UreqTransport};
