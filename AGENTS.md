# AGENTS.md — operating instructions for automated agents

This repository follows the r8n112 engineering standard. If you are an
autonomous coding agent, read this file in full before making any change.

## What this project is

`hcloud-metadata` reads the Hetzner Cloud instance metadata service (IMDS) at
`http://169.254.169.254` and exposes it as a small Rust library plus a CLI. It
requires no credentials and only performs link-local `GET` requests.

## Non-negotiables

- **TDD.** Write a failing test first, make it pass with the smallest change,
  then refactor. No behaviour without a test.
- **No `unsafe`** (`unsafe_code = "forbid"`); do not weaken that.
- **No panics on input.** Never `unwrap`/`expect`/`panic!` on network or user
  data. Use [`Error`](src/error.rs); a missing key is `Error::NotFound`.
- **Document the public API.** Every public item has a doc comment; fallible
  functions document `# Errors`.
- **Do not weaken lints** in `Cargo.toml`; justify any `#[allow(...)]` inline.
- **Minimal dependencies.** New deps must be justified; `cargo-deny` enforces
  the license allow-list.
- **Tests never touch the network.** Implement `Transport` with an in-memory
  mock as in `tests/metadata.rs`.
- **Small signed commits.** `git commit -s`, one logical change, conventional
  message.

## Commands

```sh
just fmt    # rustfmt + taplo
just lint   # clippy --all-targets --all-features -- -D warnings
just test   # cargo test (unit + integration + doctests)
just audit  # cargo-deny
just ci     # everything, as CI runs it
```

## Architecture

- `src/lib.rs` — crate root, re-exports, crate docs.
- `src/client.rs` — `MetadataClient`, base-URL handling, `get_text`/`get_json`.
- `src/transport.rs` — `Transport` trait, `HttpResponse`, `UreqTransport`.
- `src/metadata.rs` — `Metadata` model, `get`, `field`, `user_data`, path consts.
- `src/error.rs` — typed `Error`.
- `src/main.rs` — thin `clap` CLI.
- `tests/` — integration tests with `MockTransport`.

## How to add a metadata field or endpoint

1. Pick a task from `BACKLOG.md`.
2. Add a failing test in `tests/metadata.rs` (or a unit test in the module).
3. Implement in `src/metadata.rs` (and `src/client.rs` if a new request shape is
   needed); document it.
4. Wire into `src/main.rs` if the CLI should expose it.
5. `just ci` until green, then commit.

## Definition of done

- [ ] A test was added first and now passes.
- [ ] `cargo fmt --check`, clippy, tests and docs all pass.
- [ ] Public items documented; no new `unwrap`/`expect` on input.
- [ ] `cargo-deny` clean; no unjustified dependency.
- [ ] Commit is signed, conventional, and references the backlog task id.

## Backlog

`BACKLOG.md` holds self-contained tasks with acceptance criteria. Work them in
order unless told otherwise; do not mark a task done until CI is green.
