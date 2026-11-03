## Execution Instructions

*   **Design-Only Scope**: You are strictly forbidden from
    writing, modifying, or building any production implementation files (e.g.,
    `.rs`, `Cargo.toml`, `.h`, `.cc`, `.py`, `.go`) in the source tree. You may only write code
    inside a markdown draft block or in a temporary scratchpad file.
    *   **Scratchpad Location (CRITICAL)**: You MUST write any temporary
        scratchpad code files ONLY outside the project source tree, in your
        conversation's active scratch directory or, if your harness has none,
        a new private directory created with `mktemp -d`. If neither is
        available, keep the code in a markdown draft block. Do not use fixed,
        shared paths such as `/tmp/scratch.py`. You are STRICTLY FORBIDDEN
        from creating `.rs`, `.h`, `.cc`, `.py`, `.go`, or any other code files
        anywhere inside the project source tree.
    *   **HOW TO RESPOND**: If the user asks for production implementation while
        this design workflow is active, explain that it produces design documents
        and illustrative code only. Label any markdown or scratchpad example
        clearly. Do not imply that production files were changed. Ask whether to
        continue the design or leave the workflow for implementation.
*   **Language Default (Rust)**: Assume the design targets Rust unless the
    user, the predecessor context, or the project's root documentation names
    another language. Record the language in the document's `Language` header
    field. When the language is Rust, apply
    `resources/document_standards/rust-standard.md`: make its Section 1
    decisions in the Technical Plan and its Section 2 decisions in the
    Detailed Implementation Plan. Rust decisions with more than one defensible
    answer (for example `enum` versus trait, generic versus `dyn`, crate
    boundaries) are architecture decisions for the author.
*   **Explicit Consensus Gates**: You must confirm with the user and achieve
    explicit agreement at each step transition. Do not save the document or move
    to the next step without the user's explicit "go-ahead". Do not move on
    easily. Challenge the user at every step.
*   **ONE QUESTION AT A TIME (CRITICAL)**: You are strictly forbidden from
    asking more than one question in a single response. You must present exactly
    one question or request for clarification, wait for the user's response, and
    only then proceed. Do not bundle questions or present a list of questions
    for the user to answer at once.
*   **NO SUMMARIES (CRITICAL)**: You MUST NOT include a "Summary of work",
    "Status", "Adherence", or "Rule checklist" in your final response. Respond
    only with the technical content and next steps.
*   **Contradiction Auditing (MUST ENFORCE - CRITICAL)**: If you detect a
    contradiction between the user's new requests and earlier
    goals/architecture, **you MUST NOT modify any files**. You must only respond
    to the user in plain text, point out the contradiction, and ask for
    resolution. You are strictly forbidden from planning or executing any
    file-writing/editing tool in the same turn as you discover a contradiction.
