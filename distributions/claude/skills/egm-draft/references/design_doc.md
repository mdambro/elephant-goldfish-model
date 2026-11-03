# [Feature/Project Name]

*   **Author:** [Author Name or GitHub Handle]
*   **Status:** [Draft | Proposed | In Review | Approved]
*   **Date:** [YYYY-MM-DD]
*   **Language:** [Rust (default) | Other language and reason]

--------------------------------------------------------------------------------

## Overview

<!-- Provide a high-level summary of the feature, the business or technical
problem it solves, and its context within the system. Keep it concise so both
humans and agents can quickly parse the main objective.
-->

## The Problem

[Summarize the problem and the proposed solution here.]

## Goals

<!--
What will this feature achieve? Be specific, measurable, and clear.
These goals represent the absolute criteria for success.
-->

*   Goal 1
*   Goal 2

## Non-Goals

<!--
Explicitly state what is out of scope. This is crucial to prevent feature creep
and to define strict boundaries for the implementation agent.
-->

*   Non-Goal 1
*   Non-Goal 2

## Open Questions

<!--
List unresolved design questions, technical unknowns, or dependencies requiring
investigation. Unanswered questions are retained here across drafting sessions
to preserve conversational memory and prevent future sessions from re-asking
questions that were already discussed. Remove answered questions as they are
resolved.
-->

*   **Question 1**: [Description of technical unknown or open question]
*   **Question 2**: [Description of technical unknown or open question]

## Critical User Journeys (CUJs)

<!--
Define the end-to-end user journeys this design enables. Each CUJ must contain:

1. Given: Preconditions, starting state, and environment configurations.
2. When: The trigger, user action, or system event.
3. Then: The expected outcome, verification criteria, and postconditions.
-->

### CUJ 1: [Journey Title]

*   **Given**: [Preconditions]
*   **When**: [Actions]
*   **Then**: [Expected Outcomes]

### CUJ 2: [Journey Title]

*   **Given**: [Preconditions]
*   **When**: [Actions]
*   **Then**: [Expected Outcomes]

## Technical Plan (Architecture)

<!-- Detail the high-level design, architectural choices, system data flows, and
component interactions. Include ASCII or Mermaid diagrams to clearly model the
relationships.
-->

[Explain your architectural choices, data structures, and system component
mapping here.]

```text
[Optional ASCII Architecture Diagram]
```

### Rust Design Decisions

<!-- Required when Language is Rust; omit for other languages. Record each
decision from Section 1 of resources/document_standards/rust-standard.md that
applies, with its reason. Use a pattern only when the design states its
problem.
-->

*   **Crates & Layers**: [Default workspace `domain`, `application`,
    `infrastructure` (plus `proto` for gRPC and one crate per frontend), or a
    reduced layout and why. Crate of each new type.]
*   **Delivery**: [REST with axum, gRPC with tonic and the `proto` crate,
    Tauri commands, Dioxus server functions, or a combination; ports and
    listeners.]
*   **Responsibilities**: [One struct per use case (`execute`) and per
    validation rule; adapters only translate.]
*   **Persistence**: [`tokio-postgres` with `deadpool-postgres`, transaction
    sharing across repositories, migrations tool.]
*   **Validation**: [Domain newtype constructors for format invariants;
    `validations_rs` rules for business rules, sync before async; mapping of
    rule codes to the transport.]
*   **Domain Model**: [Enums for closed sets and runtime states, newtypes for
    validated values, typestate for fixed in-process flows.]
*   **Errors**: [Error type per layer, typed versus opaque, and where errors
    map to external codes.]
*   **Polymorphism**: [Enum, generic trait, or `dyn Trait` for each extension
    point, and why.]
*   **Ownership & Concurrency**: [Owners of long-lived state, sharing
    mechanism, `async` runtime or none, `Send + Sync` needs.]
*   **Dependencies & Unsafe**: [New crates with justification; `unsafe` is
    none, or isolated with documented invariants.]

## Alternatives Considered

<!-- Document other architectural approaches or designs that were evaluated and
rejected. Explain the pros, cons, and the technical reasoning for why the chosen
plan is superior.
-->

### Alternative A: [Alternative Title]

*   **Pros**: [List benefits]
*   **Cons**: [List drawbacks]
*   **Reason for Rejection**: [Why this option was not selected (e.g., "Rejected by author: technical rationale"). Do not attribute a rejection to a maintainer without linking their issue, discussion, or PR.]

--------------------------------------------------------------------------------

## Non-Functional Requirements

<!-- Analyze non-functional concerns only if they represent critical risks for
this design:

- Security & Privacy: (Data sensitivity, auth/auth, secrets management).
- Performance & Latency: (Execution runtime, storage footprint, LLM token
  overhead).
- Rollout & Rollback: (Feature flags, backward compatibility, rollback
  commands).
-->

[Detail any critical non-functional requirements here. If none are critical,
this section can be omitted or stated as "None".]

--------------------------------------------------------------------------------

### Additional Resources

<!-- Specify the external resources and context that were used during the design
discussion. Optional. If none were provided omit this section.-->

*   Important project docs referenced
*   Important docs/ referenced
*   Related work

### Future Improvements

<!-- Specify areas that have room for future improvement here. If none were
provided omit this section.-->

*   **The idea**: What could be improved?

--------------------------------------------------------------------------------

## Detailed Implementation Plan

<!-- Specify exactly which files must be created or modified. Do not use vague
language or TBDs.
-->

### Target Files to Create/Modify

#### 1. `[File Path 1]` ([Create | Modify])

*   **Type / Scope**: [e.g., Rust Library Crate Module, Rust Binary, Cargo
    Manifest, Python Service, TypeScript Module]
*   **Proposed Changes**: [Describe the new APIs, class/function signatures, or
    behavioral logic here. Focus on the minimal required details for
    implementation. For Rust, write signatures in Rust syntax and list error
    variants, trait bounds, derives, and Cargo changes (Section 2 of the Rust
    Standard).]

#### 2. `[File Path 2]` ([Create | Modify])

*   **Type / Scope**: [e.g., Unit Test, E2E Test Driver]
*   **Proposed Changes**: [Describe the test coverage, expected test cases, or
    mock assertions.]

### Verification Plan

<!-- Specify what unit, integration, or manual tests must pass to validate the
implementation. What metrics should be measured to verify success?
-->

*   **Compilation/Build**: [e.g., `cargo build --all-targets`, `npm run build`]
*   **Unit Tests**: [e.g., `cargo test --all-features`, `pytest`, `npm test`]
*   **Static Analysis & Linting**: [e.g., `cargo fmt --check` and
    `cargo clippy --all-targets --all-features -- -D warnings`, `ruff check`]
*   **Test Coverage Requirement**: [e.g., 85%+ branch coverage]
*   **E2E/Integration**: [e.g., Integration test command or automated eval runner]

--------------------------------------------------------------------------------

<!-- Provenance: Indicates initial structural drafting via EGM. -->

*Drafted with EGM*
