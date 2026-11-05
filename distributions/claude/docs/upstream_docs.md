# Using the Upstream Documentation

This repository is a fork of
[`google/elephant-goldfish-model`](https://github.com/google/elephant-goldfish-model).
The website at [egm.dev](https://egm.dev) and the upstream repository document
Google's plugin, not this one. Their explanations of the workflow still apply,
but their install commands and plugin names do not.

**If the two disagree, this repository wins.** The website tracks upstream, so it
can also describe features that reach this fork only after the next sync.

## Translate What You Read

Apply this table to anything you read on egm.dev or in the upstream repository.

| Upstream says | Use this instead |
| --- | --- |
| `Install https://github.com/google/elephant-goldfish-model` | `Install https://github.com/mdambro/elephant-goldfish-model` |
| `agy plugin install https://github.com/google/elephant-goldfish-model` | `agy plugin install https://github.com/mdambro/elephant-goldfish-model` |
| `claude plugin marketplace add google/elephant-goldfish-model` | `claude plugin marketplace add mdambro/elephant-goldfish-model` |
| `claude plugin install egm@egm` | `claude plugin install egm-rust@egm-rust` |
| `codex plugin marketplace add google/elephant-goldfish-model` | `codex plugin marketplace add mdambro/elephant-goldfish-model` |
| `codex plugin add egm@egm` | `codex plugin add egm-rust@egm-rust` |
| `omp plugin install egm@egm` | `omp plugin install egm-rust@egm-rust` |
| `muse plugins approve egm` | `muse plugins approve egm-rust` |
| `/egm:egm-draft` (any skill, with a prefix) | `/egm-rust:egm-draft` |
| `egm:egm-zoom-out` (any reviewer) | `egm-rust:egm-zoom-out` |
| `copilot --agent egm:egm` | `copilot --agent egm-rust:egm-rust` |
| `opencode --agent egm` | `opencode --agent egm-rust` |
| `.../plugins/cache/egm/egm/<version>/` | `.../plugins/cache/egm-rust/egm-rust/<version>/` |
| `~/.gemini/config/plugins/egm/` | `~/.gemini/config/plugins/egm-rust/` |
| `~/.cursor/plugins/local/egm` | `~/.cursor/plugins/local/egm-rust` |

The website shows skills without a prefix, such as `/egm-draft`. Add the prefix
where your host needs one, as the [README](../README.md#try-your-first-design)
explains: `/egm-rust:egm-draft` in Claude Code and the agy CLI.

For every host's install command, see [Harnesses](harnesses.md).

## What Stays the Same

*   The Elephant and Goldfish model, and the human approval gates.
*   The workflow: `egm-brainstorm`, `egm-draft`, `egm-review`, and
    `egm-drift-report`, with `egm-resolve`, `egm-bootstrap`, and
    `egm-review-loop` as supporting skills.
*   Skill names. Only the plugin prefix in front of them changes.
*   Reviewer names, apart from the Rust reviewer added below.

## What This Fork Adds

The website does not mention any of these, because upstream does not have them.
[Rust by Default](rust.md) covers them in full.

*   **Rust is the default language.** `egm-draft` records a `Language` field and
    a `Rust Design Decisions` subsection unless you name another language.
*   **One more reviewer.** `egm-rust-reviewer` joins Stage 7 and Stage 9 of
    `egm-review` and applies the
    [Rust Standard](../resources/document_standards/rust-standard.md).
*   **An onboarding guide** in Spanish, in
    [`docs/onboarding`](https://github.com/mdambro/elephant-goldfish-model/blob/main/docs/onboarding/README.md).

## Upstream Links That Stay Upstream

Numbered references such as issue #105 or PR #103 in these documents point to the
upstream tracker, where that history lives. Read them as background: a decision
recorded there can be changed here.

The website's footer and its privacy and terms links describe the website, not
this plugin. For the plugin, read [Privacy](privacy.md).

## Run Next to the Upstream Plugin

Both plugins can be installed on one machine, because their marketplaces have
different names. Keep only one active in a project, or every skill and reviewer
appears twice. To use this fork in one project while the upstream plugin stays
active elsewhere, run these from that project:

```sh
claude plugin marketplace add mdambro/elephant-goldfish-model --scope project
claude plugin install egm-rust@egm-rust --scope project
claude plugin disable egm@egm --scope project
```

Start a new session afterwards. If GitHub SSH access is not set up, pass
`https://github.com/mdambro/elephant-goldfish-model.git` to the first command.
