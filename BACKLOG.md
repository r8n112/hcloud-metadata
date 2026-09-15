# Backlog

Self-contained tasks for `hcloud-metadata`, ordered by priority. Pick the first
task whose status is `todo`, satisfy every acceptance criterion, then mark it
`done`.

## T001 — Typed `private-networks` model

- Status: todo
- Depends on: none

`Metadata::private_networks` is currently raw `serde_json::Value`. Model it as a
typed list so callers do not have to parse JSON.

Expected shape (verify against a real instance before finalising):

```json
[{"ip":"10.0.0.2","alias_ips":["10.0.0.3"],"mac_address":"86:00:00:...",
  "network_id":123456,"network":"10.0.0.0/16"}]
```

Acceptance criteria:
- [ ] a failing-first test deserialises a representative `private-networks` array
- [ ] `Metadata::private_networks` is a typed `Vec<PrivateNetwork>`
- [ ] unknown fields do not break deserialisation
- [ ] `just ci` is green

## T002 — Timeouts and bounded retry

- Status: todo
- Depends on: none

A hung IMDS call should not hang the process. Add a configurable request timeout
and a single bounded retry for transient transport failures.

Acceptance criteria:
- [ ] `MetadataClient` can be configured with a timeout
- [ ] a failing-first test proves retry behaviour via the mock transport
- [ ] default timeout is documented
- [ ] `just ci` is green

## T003 — CLI: list available keys

- Status: todo
- Depends on: none

`hcloud-metadata get <key>` fails at request time for an unknown key. Add a
`keys` subcommand listing the keys the crate models.

Acceptance criteria:
- [ ] `hcloud-metadata keys` prints the known keys, one per line
- [ ] a test asserts the list is non-empty and stable
- [ ] `just ci` is green

## T004 — Opt-in live integration test

- Status: todo
- Depends on: none

Add an integration test that hits the real IMDS, skipped unless an env var (for
example `HCLOUD_METADATA_LIVE=1`) is set, so it only runs inside a Hetzner
instance.

Acceptance criteria:
- [ ] test is skipped by default and never runs in CI
- [ ] documented in README
- [ ] `just ci` stays green without the env var

## T005 — MSRV CI job

- Status: todo
- Depends on: none

The manifest declares `rust-version = "1.74"`. Add a CI job that builds with the
declared MSRV so it cannot silently drift.

Acceptance criteria:
- [ ] CI has an MSRV job pinned to the version in `Cargo.toml`
- [ ] the job passes on `main`
