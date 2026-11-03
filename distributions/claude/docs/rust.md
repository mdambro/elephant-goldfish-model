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
used without a problem that needs them.

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
- Workspace: crates/domain (no I/O dependencies), crates/application,
  crates/infrastructure (binary). Dependencies point inward only.
- Errors: thiserror enums per crate; anyhow only in the binary's main.
- Async: tokio; shared ports are traits with Send + Sync.
- Lints: configured in [workspace.lints]; CI runs
  cargo fmt --check, cargo clippy --all-targets --all-features -- -D warnings,
  and cargo test --all-features.
- Follow the EGM Rust Standard for patterns and anti-patterns.
```

For a project context skill (see [Getting Started](getting_started.md)), link
this file, the workspace `Cargo.toml`, and the approved designs in
`docs/designs/`.
