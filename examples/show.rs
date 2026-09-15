//! Prints the instance metadata as JSON.
//!
//! Run this from inside a Hetzner Cloud instance:
//!
//! ```sh
//! cargo run --example show
//! ```

use hcloud_metadata::{metadata, MetadataClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MetadataClient::new();
    let info = metadata::get(&client)?;
    println!("{}", serde_json::to_string_pretty(&info)?);
    Ok(())
}
