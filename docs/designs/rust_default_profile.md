# Rust Default Profile

*   **Author:** [@mdambro](https://github.com/mdambro)
*   **Status:** Proposed
*   **Date:** 2026-11-03

--------------------------------------------------------------------------------

## Overview

This fork of EGM is used mainly for Rust projects. Upstream EGM is
language-neutral: it never states which language a design targets, and none of
its reviewers checks language-specific design decisions. This design makes Rust
the default language of the workflow and adds a Rust reviewer and rubric, while
keeping other languages possible.

## The Problem

Three gaps lead to Rust designs that leave decisions to the implementer:

1.  **No language decision**: `egm-draft` never records the target language, so
    the implementation agent infers it, and reviewers cannot tell which
    idioms apply.
2.  **No Rust rubric**: The existing standards ask for file lists, signatures,
    and tests, but not for the decisions that shape Rust code: ownership of
    shared state, error types per layer, `enum` versus trait, generic versus
    `dyn`, `async` runtime, and crate boundaries. Plans can pass the
    Implementation Readiness Standard and still leave these open.
3.  **Project instructions do not reach reviewers**: Claude reviewer adapters
    set `omitClaudeMd: true`, and every Goldfish reads only the document,
    standards, and Goldfish context. A "use Rust" rule in a project's
    `CLAUDE.md` or `AGENTS.md` reaches the Elephant but never the reviewers.
    The rule must live in a standard that reviewers load.

## Goals

*   Designs drafted with `egm-draft` record a `Language` field that defaults to
    Rust.
*   Rust designs state the architecture-level Rust decisions in the Technical
    Plan and the code-level Rust details in the implementation plan.
*   `egm-review` Stages 7 and 9 include a Rust reviewer that applies a written
    rubric, including checks against unjustified patterns.
*   Non-Rust designs still work: the Rust reviewer returns "not applicable".
*   The author's defaults are explicit and checked: Clean Architecture crates
    (`domain`, `application`, `infrastructure`, and `proto` for gRPC), one
    responsibility per type, and the stack (`tokio`, `axum`, `tonic`,
    `tokio-postgres`, Tauri, Dioxus, `validations_rs`).
*   The fork stays easy to rebase on upstream: Rust content lives in new files,
    and edits to upstream files are small.
*   A developer new to Rust can onboard from this repository alone: install
    commands point to this fork, and `docs/onboarding/` holds a guide and a
    practice project that follow the Rust Standard.

## Non-Goals

*   An implementation skill that writes Rust code. Phase 5 stays as upstream
    defines it; the approved design carries the Rust decisions.
*   A general language-profile mechanism for other languages.
*   Changes to `egm-brainstorm`, `egm-bootstrap`, `egm-resolve`, or
    `egm-drift-report`.

## Critical User Journeys (CUJs)

### CUJ 1: Draft a Rust Feature

*   **Given**: A project with no language stated and the fork installed.
*   **When**: The user runs `/egm-draft`.
*   **Then**: The draft header shows `Language: Rust (default)`, the Technical
    Plan contains a `Rust Design Decisions` subsection, and decisions with more
    than one defensible answer are presented to the author.

### CUJ 2: Review a Rust Design

*   **Given**: A Rust design document.
*   **When**: The user runs `/egm-review` with all stages.
*   **Then**: Stage 7 and Stage 9 reports include `egm-rust` findings graded by
    the Rust Standard.

### CUJ 3: Review a Non-Rust Design

*   **Given**: A design whose `Language` field names Go.
*   **When**: The user runs `/egm-review`.
*   **Then**: `egm-rust` reports that the standard does not apply, with no
    findings.

## Technical Plan (Architecture)

```text
resources/execution_instructions.md   Language Default rule (read by egm-draft)
            │
            ▼
skills/egm-draft ──► design doc: Language field + Rust Design Decisions
                                         │
                                         ▼
skills/egm-review Stage 7 / Stage 9 ──► egm-rust (agents/rust.md)
                                         │ loads
                                         ▼
                 resources/document_standards/rust-standard.md
```

*   **Rust Standard** (new): Applicability rules, feature-level decisions
    (including single responsibility and validation placement),
    zero-decision implementation items, pattern selection table,
    anti-patterns, default architecture, default stack, and severity guide.
    It is the single source of Rust rules. Stack versions are recorded with
    a date, because they age.
*   **`egm-rust` persona** (new): A read-only Goldfish that applies the Rust
    Standard at the altitude of the reviewed range, verifies claims against the
    codebase, and reports unjustified patterns as well as missing ones.
*   **Harness adapters** (new): Claude, Cursor, and Copilot adapters for
    `egm-rust`, copied from the existing reviewer adapters.
*   **Wiring** (edits): The Language Default rule in the execution
    instructions; the `Language` field and `Rust Design Decisions` subsection
    in the design template; one step each in `egm-draft` Steps 1, 3, and 5;
    `egm-rust` added to the Stage 7 and Stage 9 reviewer lists in `egm-review`;
    `rust` and `rust-standard` shortnames in `egm-review-loop`.

The `egm-review` skill body is close to the 24,000-byte limit enforced by
`tests/unit/test_skill_load_limits.py`, so the applicability check lives in the
persona and standard instead of the skill.

## Alternatives Considered

### Alternative A: Rust Rules Only in Project `CLAUDE.md` or `AGENTS.md`

*   **Pros**: No change to EGM.
*   **Cons**: Reviewers never read these files, so review stages would not check
    Rust decisions.
*   **Reason for Rejection**: Rejected by author: the rule would reach the
    Elephant only.

### Alternative B: Add Rust Rules to the Existing Standards

*   **Pros**: No new persona or adapters.
*   **Cons**: Every upstream edit to those standards would conflict with the
    fork, and language rules would mix with language-neutral rules.
*   **Reason for Rejection**: Rejected by author: higher rebase cost and less
    clear ownership of the rules.

### Alternative C: Stack Rules in a Separate Standard

*   **Pros**: The language rules and the author's stack could change
    independently.
*   **Cons**: `egm-review-loop` maps one standard to each reviewer, so a second
    standard needs a second reviewer or a change to the loop.
*   **Reason for Rejection**: Rejected by author: one reviewer and one
    standard are simpler; the stack is a separate section.

### Alternative D: General Language Profiles

*   **Pros**: Supports any language with a profile file.
*   **Cons**: Needs a selection mechanism and profiles nobody uses.
*   **Reason for Rejection**: Rejected by author: YAGNI for a Rust-first fork.
    The applicability section leaves room to add this later.

--------------------------------------------------------------------------------

## Non-Functional Requirements

*   **Performance & Latency**: Stages 7 and 9 dispatch one more reviewer each,
    adding one parallel subagent call per stage.
*   **Rollout & Rollback**: Reverting this change restores upstream behavior;
    no state is stored.

--------------------------------------------------------------------------------

## Detailed Implementation Plan

### Target Files to Create/Modify

#### 1. `resources/document_standards/rust-standard.md` (Create)

*   **Type / Scope**: Runtime rubric.
*   **Proposed Changes**: Sections 0 to 5 as described in the Technical Plan.

#### 2. `agents/rust.md` (Create)

*   **Type / Scope**: Canonical reviewer persona, name `egm-rust`.
*   **Proposed Changes**: Core mandates shared with other Goldfish; `BRAINSTORM`
    and `REVIEW` modes; reads the Rust Standard before every audit.

#### 3. `.claude-plugin/agents/rust.md`, `.cursor-plugin/agents/rust.md`, `com.github.copilot/agents/egm-rust.agent.md` (Create)

*   **Type / Scope**: Harness adapters.
*   **Proposed Changes**: Same restrictions as the existing reviewer adapters,
    pointing to `agents/rust.md`.

#### 4. `.claude-plugin/plugin.json`, `adapters/opencode/install.py`, `scripts/pack_plugin.py`, `com.github.copilot/agents/egm.agent.md` (Modify)

*   **Type / Scope**: Registration.
*   **Proposed Changes**: Register the adapter, add `rust` to reviewer lists and
    required bundle files, add `docs/rust.md` to the bundle, and update the
    reviewer count.

#### 5. `resources/execution_instructions.md`, `resources/README.md` (Modify)

*   **Type / Scope**: Runtime rules.
*   **Proposed Changes**: Add the Language Default rule; add `.rs` and
    `Cargo.toml` to the design-only file list; list the new standard.

#### 6. `skills/egm-draft/SKILL.md`, `skills/egm-draft/references/design_doc.md` (Modify)

*   **Type / Scope**: Drafting workflow and template.
*   **Proposed Changes**: Resolve and record the language; fill `Rust Design
    Decisions`; require Section 2 items for Rust plans; Rust defaults in
    examples and the Verification Plan.

#### 7. `skills/egm-review/SKILL.md`, `skills/egm-review-loop/SKILL.md` (Modify)

*   **Type / Scope**: Review workflow.
*   **Proposed Changes**: Add `egm-rust` with `rust-standard` to Stages 7 and 9;
    add shortnames and a recommendation.

#### 8. `docs/rust.md`, `docs/designs/README.md`, `docs/copilot.md`, `docs/opencode.md`, `README.md` (Create/Modify)

*   **Type / Scope**: Documentation.
*   **Proposed Changes**: Usage guide with a project template; index entry;
    reviewer tables and counts; a short README section.

#### 9. `docs/onboarding/` and install commands (Create/Modify)

*   **Type / Scope**: Onboarding and installation.
*   **Proposed Changes**: Add the onboarding guide (PDF), its practice
    workspace, and an index. Point the install commands in `README.md`,
    `docs/claude.md`, `docs/codex.md`, and `docs/harnesses.md` to this fork,
    because the marketplace manifests use `source: "./"` and install whichever
    repository was added.

#### 10. `tests/unit/test_harnesses.py`, `tests/unit/test_rust_profile.py`, `distributions/claude/` (Modify/Create)

*   **Type / Scope**: Tests and generated runtime.
*   **Proposed Changes**: Add `rust` to `REVIEWERS`; new tests check the wiring;
    regenerate the published Claude runtime with `scripts/pack_plugin.py`.

### Verification Plan

*   **Unit Tests**: `python3 -m pytest tests/unit`.
*   **Manual**: Run CUJ 1 to CUJ 3 in Claude Code against a sample Rust project.

--------------------------------------------------------------------------------

*Drafted with EGM*
