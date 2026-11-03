# Rust by Default

This distribution of EGM treats Rust as the default language. The
[design](https://github.com/mdambro/elephant-goldfish-model/blob/main/docs/designs/rust_default_profile.md) explains why; this page explains how
to use it.

## What Changes

| Stage | Behavior |
| --- | --- |
| `/egm-draft` | Records `Language: Rust (default)` unless you, the brief, or the project's root docs name another language. Adds a `Rust Design Decisions` subsection to the Technical Plan and asks you to choose when a Rust decision has more than one defensible answer. Implementation plans include Cargo changes, Rust signatures, error enums, derives, and `cargo` verification commands. |
| `/egm-review` | Stage 7 (feature spec) and Stage 9 (implementation plan) add the `egm-rust` reviewer, which applies the [Rust Standard](../resources/document_standards/rust-standard.md). |
| `/egm-review-loop` | Accepts `rust` as a reviewer and `rust-standard` as a standard. |

The Rust Standard checks two directions: Rust decisions left open, and patterns
used without a problem that needs them. It also sets the defaults below; a
design may change them, but it must say why.

## Defaults

*   **Layout**: a Cargo workspace with `domain`, `application`, and
    `infrastructure` crates, plus a `proto` crate when the service uses gRPC
    instead of (or next to) REST, and one crate per Tauri or Dioxus frontend.
    Dependencies point inward.
*   **Single responsibility**: one struct per use case with one `execute`
    method, one struct per validation rule, one repository trait per
    aggregate, adapters that only translate.
*   **Stack**: `tokio`, `axum` 0.8, `tonic` 0.14 with `prost`,
    `tokio-postgres` with `deadpool-postgres`, Tauri 2, Dioxus 0.7,
    `thiserror` (and `anyhow` only in `main`).
*   **Validation**: domain newtypes for format invariants; `validations_rs`
    rules in `application` for business rules.

The full rules are in Sections 5 and 6 of the
[Rust Standard](../resources/document_standards/rust-standard.md).

## Use Another Language

State it when you draft ("this service is in Go"), or put it in the project's
root `README.md` or `AGENTS.md`. The draft records it in the `Language` field,
and `egm-rust` reports that the standard does not apply.

## Set Up a Rust Project

Reviewers read only the design document, the standards, and the files you pass
as Goldfish context. They do not read `CLAUDE.md` or `AGENTS.md`. Put
project-specific Rust conventions in a checked-in file and pass it as Goldfish
context, or write them into each design.

A starting `AGENTS.md` (or `CLAUDE.md`) section for the Elephant and the
implementation agent:

```markdown
## Rust Conventions

- Language: Rust, edition 2021, MSRV <version>.
- Workspace: crates/domain (no I/O), crates/application (use cases, ports,
  validations_rs rules), crates/infrastructure (axum or tonic adapters,
  tokio-postgres repositories, main), crates/proto (only with gRPC).
  Dependencies point inward only; prost types never leave infrastructure.
- One responsibility per type: one use case struct with one `execute`, one
  validation rule per struct, one file per public type.
- Errors: thiserror enums per crate; anyhow only in main.
- Async: tokio; ports are traits with Send + Sync.
- Database: tokio-postgres + deadpool-postgres; $1 parameters only.
- Lints: configured in [workspace.lints]; CI runs
  cargo fmt --check, cargo clippy --all-targets --all-features -- -D warnings,
  and cargo test --all-features.
- Follow the EGM Rust Standard for patterns, anti-patterns, and the stack.
```

For a project context skill (see [Getting Started](getting_started.md)), link
this file, the workspace `Cargo.toml`, and the approved designs in
`docs/designs/`.
