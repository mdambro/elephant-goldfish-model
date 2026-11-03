# Rust Standard

This standard defines the quality bar for designs and implementation plans
that target Rust. In this distribution of EGM, Rust is the default language.

## Core Principle

A document passes if a competent Rust engineer, unfamiliar with the codebase,
can implement it idiomatically without guessing how data is owned, how errors
flow, how components are made polymorphic, or how work runs concurrently, and
if every design pattern it uses solves a problem the document states.

Patterns are tools, not goals. A design fails this standard both when it leaves
a Rust decision open and when it adds an abstraction that no stated
requirement needs.

## 0. Applicability (Language Default)

*   **Rust by Default**: A document targets Rust unless its `Language` header
    field or its Technical Plan names another language.
*   **Other Languages**: If the document names another language, this standard
    does not apply. The reviewer returns one line stating that and no findings.
*   **Missing Language Field**: Treat the document as Rust and report the
    missing `Language` field once as a Nit.
*   **Mixed Systems**: If only some components are Rust, apply this standard to
    those components and to their boundaries (FFI, wire formats, process
    boundaries).

## 1. Feature-Level Decisions (Architecture)

The feature specification must make these decisions explicitly when they apply.
They are architecture, so they belong above the implementation plan.

*   **Crate and Module Boundaries**: Which crates exist and which depend on
    which. Use a Cargo workspace when layers must not depend on each other (for
    example, a domain crate that cannot reach infrastructure). Cargo enforces
    that direction and rejects cycles. Do not split into crates without a
    boundary or build-time reason; a single crate with modules is acceptable.
*   **Domain Model**: Make invalid states unrepresentable.
    *   Use `enum` for closed sets of alternatives and for entity states loaded
        at runtime (from storage, the network, or user input).
    *   Use newtypes with private fields and fallible constructors for
        validated values and identifiers ("parse, don't validate").
    *   Use typestate (one type per state) only for fixed, in-process flows
        such as builders, protocols, and multi-step operations.
    *   Reject structs whose fields are valid only in some states (boolean
        flags plus optional fields that stand for a missing enum).
*   **Error Strategy**: Each layer exposes its own error type, normally an
    `enum` whose variants callers can match. Library and domain crates return
    typed errors (`thiserror` is the common choice). Only the outermost binary
    may collapse errors into an opaque type (`anyhow` or `Box<dyn Error>`).
    State where errors are mapped to external codes (HTTP status, exit codes).
    Expected failures never `panic!`.
*   **Polymorphism**: Use an `enum` when the set of variants is closed and
    owned by this codebase. Use a `trait` when other code must add
    implementations (repositories, gateways, plugins). Prefer generics (static
    dispatch); use `dyn Trait` only for runtime selection, heterogeneous
    collections, or dependency injection at the composition root. State the
    choice and the reason.
*   **Ownership and Sharing**: State who owns each long-lived value. Shared
    state uses `Arc` (with `Mutex` or `RwLock` when mutated) or message passing;
    name which. Model relationships between entities with identifiers, not
    graphs of references.
*   **Concurrency and Async**: State whether the feature needs `async`. If it
    does, name the runtime (for example `tokio`). Traits shared across tasks
    require `Send + Sync`. Blocking work (file I/O, CPU-bound work, blocking
    clients) is isolated from async executors.
*   **Unsafe Code**: None by default; crates that do not need it declare
    `#![forbid(unsafe_code)]`. If `unsafe` is required, isolate it in a small
    module with a safe API and document the invariants each `unsafe` block
    relies on.
*   **Dependencies**: Prefer the standard library and established crates over
    custom code. Justify each new crate by need, maintenance activity, and
    license. Do not add a crate for a few lines of code.

## 2. Implementation-Level Decisions (Zero-Decision Rust)

The implementation plan must leave no Rust decision to the implementer. In
addition to the Implementation Readiness Standard, it must specify, where
applicable:

*   **Cargo Changes**: Exact crate names, version requirements, enabled
    features, and whether each dependency belongs to the workspace or a single
    crate. State the edition and `rust-version` (MSRV) for new crates.
*   **Public API Signatures**: Rust signatures for every new public function,
    method, and trait, including visibility (`pub`, `pub(crate)`), parameter
    types, return types, and trait bounds. Parameters that only read take
    borrowed types: `&str` over `&String`, `&[T]` over `&Vec<T>`, `&T` over
    `&Box<T>`. Lifetimes appear only where a type stores a reference.
*   **Error Types**: Every new error `enum` with its variants, the `From`
    conversions that enable `?` across layers, and the mapping to external
    codes.
*   **Traits**: Methods, `Send + Sync` bounds, and whether the trait is used as
    `dyn` (then it must be dyn-compatible). For async methods behind `dyn`,
    state the mechanism (for example `async-trait` or boxed futures), because
    native `async fn` in traits is not dyn-compatible.
*   **Derives and Trait Implementations**: Which of `Debug`, `Clone`, `Copy`,
    `PartialEq`, `Eq`, `Hash`, `Default`, `Serialize`, and `Deserialize` each
    new type implements. Secrets implement `Debug` and `Display` by hand so
    they do not leak into logs.
*   **Resource Cleanup**: Locks, transactions, and handles are released by RAII
    guards. The plan does not rely on `Drop` for effects that must happen (for
    example, flushing data that cannot be lost).
*   **Tests**: Unit tests in `#[cfg(test)] mod tests` next to the code,
    integration tests in `tests/`, doc tests for public APIs, and test doubles
    as trait implementations. Name the specific cases, including error paths.
*   **Verification Commands**: At minimum `cargo fmt --check`,
    `cargo clippy --all-targets --all-features -- -D warnings`, and
    `cargo test --all-features`. Add `cargo doc --no-deps` when public APIs
    change. Warnings are denied by the command or CI configuration, never by
    `#![deny(warnings)]` in source.
*   **Naming**: Follow the Rust API Guidelines: `snake_case` for functions and
    modules, `UpperCamelCase` for types, and the `as_` (cheap borrow), `to_`
    (expensive conversion), and `into_` (consumes `self`) prefixes.

## 3. Pattern Selection

Use a pattern when its problem appears in the document. Flag a pattern that is
used without its problem as well as a problem left without a pattern.

| Problem in the document | Pattern | Do not use when |
| --- | --- | --- |
| A primitive carries meaning or validation (IDs, emails, money, units). | Newtype with private field and fallible constructor. | The value has no invariant and no risk of being confused with another. |
| A type has many optional settings; Rust has no default or named arguments. | Builder (by value, or `&mut self` for conditional steps); `build()` returns `Result` when combinations can be invalid. | Two or three fields; use a constructor or `Default` with struct update syntax. |
| An algorithm must vary, or a layer must not depend on a concrete implementation. | Strategy: a trait (generic or `dyn`) or a closure bound such as `F: Fn(..)`. | Only one implementation exists and no test double is needed. |
| Actions must be stored, queued, retried, logged, or undone. | Command: a trait object, a function pointer, or a boxed closure. | The action runs once, immediately; a function call is enough. |
| A resource must be released on every exit path, including `?` and panics. | RAII guard with `Drop`; consuming methods (`commit(self)`) end its use. | The effect must happen even if the process aborts. |
| An operation is valid only in some states of a fixed flow. | Typestate: one type per state, transitions take `self`. | The state is only known at runtime; use an `enum`. |
| A `&mut self` method blocks borrowing other fields. | Split the struct into smaller structs and borrow fields independently. | Splitting creates types with no meaning of their own. |
| An enum variant must change while only `&mut` is available. | `mem::take` or `mem::replace`. | The method can take `self` by value and return the new state. |
| A fallible function consumes an argument the caller may need to retry with. | Return the consumed argument inside the error. | The argument is cheap to recreate. |
| A trait bound is repeated and hard to read (often closures returning `Result`). | A named trait with a blanket implementation. | The bound appears once. |
| Layers must not depend on each other. | Separate crates in a workspace. | The project is small and modules make the boundary clear. |

## 4. Anti-Patterns

Report these as findings:

*   **Clone to Satisfy the Borrow Checker**: `.clone()` used to silence a borrow
    error instead of borrowing, restructuring, or `mem::take`. `Arc::clone` and
    `Rc::clone` are not findings; clones that create an intended independent
    copy are not findings.
*   **Deref Polymorphism**: Implementing `Deref` to emulate inheritance or base
    classes. Use traits with default methods or explicit delegation.
*   **`#![deny(warnings)]` in Source**: Breaks builds on compiler upgrades. Deny
    warnings in CI or name specific lints in `[lints]` or `[workspace.lints]`.
*   **`#[non_exhaustive]` Inside One Workspace**: Forces wildcard arms and
    removes the compile error that points to unhandled new variants. Reserve
    it for types published to external users.
*   **Panics on Expected Paths**: `unwrap`, `expect`, indexing, or `panic!` in
    non-test code where the failure is possible, unless the document states the
    invariant that makes it impossible.
*   **Stringly-Typed Design**: `String` or `bool` used for identifiers, states,
    or units that have a fixed set of values or an invariant.
*   **Borrowed Owned Types in Parameters**: `&String`, `&Vec<T>`, or `&Box<T>`
    parameters.
*   **Global Mutable State**: Static singletons for configuration or services.
    Pass configuration as a value (often `Arc<Config>`) from the composition
    root.
*   **Speculative Abstraction**: Traits with one implementation and no test
    double, generic parameters only ever used with one type, typestate for
    runtime state, or crates split with no boundary reason. This overlaps the
    Zoom-Out Standard; report it here only for Rust-specific forms.
*   **Blocking Inside Async**: Blocking calls on an async executor thread.

## 5. Severity Guide

*   **🔴 Blocker**: A missing feature-level decision from Section 1 that changes
    the architecture (error strategy, async model, ownership of shared state,
    crate boundaries); `unsafe` without documented invariants; a domain model
    that can represent invalid states the requirements forbid.
*   **🟡 Major**: An anti-pattern in a proposed API; a missing Section 2 item
    (Cargo details, signatures, error variants, `Send + Sync`); a pattern used
    without its problem.
*   **🟢 Nit**: Derives, naming conventions, a missing `Language` field, or
    documentation of public items.

## References

*   [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
*   [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
*   [The Rust Programming Language](https://doc.rust-lang.org/book/)
