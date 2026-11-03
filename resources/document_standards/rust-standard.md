# Rust Standard

This standard defines the quality bar for designs and implementation plans
that target Rust. In this distribution of EGM, Rust is the default language,
Clean Architecture (Section 5) is the default layout, and Section 6 lists the
default stack.

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
*   **Single Responsibility (Author Preference)**: Each type has one reason
    to change. One use case is one struct with one public `execute` method.
    One validation rule is one struct. One repository trait serves one
    aggregate. Adapters (HTTP handlers, gRPC services, Tauri commands, Dioxus
    server functions) only translate between the transport and a use case.
    Each public type lives in its own file, grouped in modules by role, as
    packages are grouped in Kotlin. A use case that grows a second
    responsibility is split, and the two are composed by a third use case or
    by the caller.
*   **Validation**: Format invariants of a single value (an email, a code, a
    range) belong to a domain newtype with a fallible constructor
    (`TryFrom`, `FromStr`, or `parse`) and the domain error type. Business
    rules that combine fields, depend on state, or need I/O belong to the
    application layer as rules of the `validations_rs` crate (Section 6), run
    at the start of the use case. State which rule protects each invariant.

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

*   **Layering**: State which crate (Section 5) each new type belongs to, and
    show that no dependency points outward.

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
*   **Multi-Responsibility Types**: A use case, service, or "manager" with
    several public operations, or a rule that checks unrelated conditions.
*   **Logic in Adapters**: Business rules or validation inside axum handlers,
    tonic services, Tauri commands, Dioxus components, or server functions.
*   **Leaking Transport or Storage Types**: `prost` messages, axum extractors,
    `tokio_postgres::Row`, Tauri or Dioxus types used in `domain` or
    `application`. Map them to domain types in the adapter, with `TryFrom`
    where the conversion can fail.

## 5. Default Architecture

A service is a Cargo workspace with these crates. Use fewer only when the
document justifies it (Zoom-Out Standard, YAGNI).

| Crate | Contains | May depend on |
| --- | --- | --- |
| `domain` | Entities, value objects (newtypes), domain enums and states, domain error enums. Pure logic, no I/O. | `std`, small pure crates (for example `thiserror`, `uuid`) |
| `application` | One struct per use case with an `execute` method; ports (repository and gateway traits with `Send + Sync`); command and result types; business-rule validations (`validations_rs`); business-rule configuration as a struct injected with `Arc`; application error enums. | `domain`, `validations_rs`, `thiserror`; no transport, storage, or UI crates |
| `proto` | Only when gRPC is chosen. The `.proto` files, a `build.rs` that runs `tonic-prost-build`, and the generated code, re-exported. No hand-written logic. | `tonic`, `prost`, `tonic-prost`; build dependency `tonic-prost-build` |
| `infrastructure` | Adapters and the composition root: axum routers and handlers or tonic service implementations, `tokio-postgres` repositories, mapping between transport or storage types and domain types, error mapping to HTTP or gRPC status, configuration loading, `main`. | `application`, `domain`, `proto` (when present), the stack crates |

Rules:

*   **Dependencies point inward**: `infrastructure` → `application` →
    `domain`. `proto` is used only by `infrastructure` and by client crates.
    Generated `prost` types never appear in `application` or `domain`.
*   **REST or gRPC**: The document states the transport. REST uses axum
    handlers. gRPC adds the `proto` crate and tonic services. Both may be
    served from one process; the document states whether they share a port
    (tonic `Routes::into_axum_router`) or use separate listeners.
*   **Contracts**: For gRPC, the `.proto` files are the public contract.
    Changes follow protobuf compatibility rules: never reuse or renumber a
    field; mark removed fields `reserved`.
*   **Frontends**: A Tauri or Dioxus app is its own crate in the workspace. It
    is a delivery adapter, like `infrastructure`: Tauri commands and Dioxus
    server functions call use cases and contain no business rules. A frontend
    that runs on another machine calls the API (REST or gRPC) through a
    client and does not depend on `application`. Dioxus web (WASM) clients
    need gRPC-Web for gRPC.

## 6. Default Stack

Versions as of November 2026; the document states the exact version
requirements it uses. A different crate for one of these roles is an
architecture decision for the author.

*   **Async runtime: `tokio` 1.x.** Binaries enable `rt-multi-thread` and
    `macros` (or `full`); libraries enable only the features they use.
    Blocking calls go to `spawn_blocking`; CPU-bound work goes to a dedicated
    pool (for example `rayon`). Prefer `std::sync::Mutex` for shared state;
    use `tokio::sync::Mutex` only when a guard is held across `.await`.
*   **HTTP: `axum` 0.8** with `tower-http` middleware (tracing, timeouts,
    CORS). Share state with the `State` extractor (type-checked at compile
    time), not `Extension` (fails at runtime with 500); wrap shared state in
    `Arc`; give handlers sub-states with `FromRef`. Handlers return
    `Result<T, ApiError>` where `ApiError` implements `IntoResponse` and maps
    every application error variant to a status code. Body extractors (`Json`)
    come last. Path parameters use `/{id}`.
*   **gRPC: `tonic` 0.14 with `prost` 0.14.** Code generation runs in the
    `proto` crate's `build.rs` with `tonic-prost-build`, which needs `protoc`
    on the build machine (state how it is installed in CI). Service
    implementations live in `infrastructure` and map application errors to
    `tonic::Status` codes (for example `InvalidArgument`,
    `FailedPrecondition`, `NotFound`, `Internal`).
*   **PostgreSQL: `tokio-postgres` 0.7.** `connect` returns a `Client` and a
    `Connection`; the `Connection` must be spawned. `tokio-postgres` has no
    pool: use `deadpool-postgres` (or state another pool). Queries use `$1`
    parameters, never string formatting. Enable only the type features in
    use (`with-uuid-1`, `with-chrono-0_4`, `with-serde_json-1`,
    `with-time-0_3`). TLS uses `postgres-native-tls` or `postgres-openssl`, or
    the document states why `NoTls` is safe. `Transaction` rolls back on drop
    unless `commit` is called, and it borrows the `Client` mutably: the
    document states how a use case that writes through several repositories
    shares one transaction (for example a unit-of-work port), and which tool
    runs schema migrations.
*   **Desktop and mobile: Tauri 2.** Commands live in a `commands` module
    (`pub` there; commands in `lib.rs` cannot be `pub`) and are all registered
    in one `generate_handler!` call. Async commands with borrowed arguments
    (`&str`, `State<'_, T>`) return `Result`. Command errors are a `thiserror`
    enum with a `Serialize` implementation. Managed state is registered with
    `manage` and needs no `Arc`; use a type alias, because a wrong `State<T>`
    type compiles and fails at runtime. Capabilities grant each window only
    the permissions it needs; custom commands are open to every window unless
    restricted in `build.rs`.
*   **Rust UI: Dioxus 0.7.** Fullstack apps declare `web`, `desktop`,
    `mobile`, and `server` features; server-only dependencies (`tokio`,
    database clients, `application`) are optional and enabled by `server`,
    and their code is behind `#[cfg(feature = "server")]`. Server functions
    are axum endpoints: they call use cases and return errors the client can
    match (`ServerFnError` or a custom serializable error). Larger apps split
    the UI and a shared API crate. Follow the Dioxus anti-pattern guide:
    stable list keys, signals instead of interior mutability in props, no
    state writes during render (use `use_effect` or `use_memo`), small pieces
    of state, non-deterministic code in hooks, and derived `PartialEq` for
    props.
*   **Errors: `thiserror`** in `domain`, `application`, and adapters;
    `anyhow` only in `main` and other top-level binaries.
*   **Business-rule validation: `validations_rs`.** One struct per rule
    implementing `BusinessLogicValidation<T>` (synchronous) or
    `AsyncBusinessLogicValidation<T>` (I/O). The use case runs synchronous
    rules first with `start_validations`, then asynchronous rules with
    `start_validations_async`, so cheap rules fail before I/O. Adapters map
    `BusinessRuleError.code` to the transport (HTTP 422, gRPC
    `FailedPrecondition` or `InvalidArgument`, a serialized Tauri or Dioxus
    error) and never parse its message.

## 7. Severity Guide

*   **🔴 Blocker**: A missing feature-level decision from Section 1 that changes
    the architecture (error strategy, async model, ownership of shared state,
    crate boundaries); `unsafe` without documented invariants; a domain model
    that can represent invalid states the requirements forbid.
*   **🟡 Major**: An anti-pattern in a proposed API; a missing Section 2 item
    (Cargo details, signatures, error variants, `Send + Sync`); a pattern used
    without its problem; a type with more than one responsibility; a
    dependency that points outward (Section 5); a Section 6 practice ignored
    without a stated reason.
*   **🟢 Nit**: Derives, naming conventions, a missing `Language` field, or
    documentation of public items.

## References

*   [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
*   [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
*   [The Rust Programming Language](https://doc.rust-lang.org/book/)
*   [axum](https://docs.rs/axum/latest/axum/),
    [tokio](https://docs.rs/tokio/latest/tokio/),
    [tokio-postgres](https://docs.rs/tokio-postgres/latest/tokio_postgres/),
    [tonic](https://github.com/hyperium/tonic),
    [Tauri 2](https://v2.tauri.app/start/),
    [Dioxus 0.7](https://dioxuslabs.com/learn/0.7/),
    [validations_rs](https://github.com/mdambro/validations_rs)
