# hcloud-metadata

A low-level Rust library and CLI for the **Hetzner Cloud instance metadata
service** (IMDS) at `http://169.254.169.254`.

Inside a Hetzner Cloud instance the IMDS exposes read-only facts about the
instance (ID, hostname, region, availability zone, public addresses, keys,
private networks) plus the cloud-init user data. This crate reads them with
minimal dependencies and no credentials.

## Status

Early (`0.1.0`). Implemented: transport abstraction, typed `Metadata` document,
single-field lookup, and cloud-init user data. See [BACKLOG.md](BACKLOG.md) for
what is planned.

## Design

- **Testable by construction.** `Transport` is a trait; production uses `ureq`,
  tests inject an in-memory mock, so the suite runs offline.
- **Minimal dependencies.** `serde`/`serde_json`, `thiserror`, `ureq`, `clap`.
- **No panics on input.** Errors are typed; missing keys map to `Error::NotFound`.
- Link-local HTTP only — no TLS, no credentials.

## Usage

```sh
cargo run -- show            # full metadata
cargo run -- get instance-id
cargo run -- user-data
cargo run -- --json show
```

As a library:

```rust
use hcloud_metadata::{metadata, MetadataClient};

let client = MetadataClient::new();
let info = metadata::get(&client)?;
println!("{:?}", info.hostname);
```

## Development

```sh
just fmt        # rustfmt + taplo
just lint       # clippy, warnings denied
just test       # tests + doctests
just audit      # cargo-deny
just ci         # everything, as CI runs it
```

## License

MIT OR Apache-2.0.
