# SPDX-License-Identifier: Apache-2.0

"""Keep the Rust default profile wired into drafting and review."""

import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[2]
STANDARD = "resources/document_standards/rust-standard.md"


def read(path: str) -> str:
  return (ROOT / path).read_text(encoding="utf-8")


def test_rust_standard_defines_required_sections() -> None:
  text = read(STANDARD)
  for heading in (
      "## 0. Applicability (Language Default)",
      "## 1. Feature-Level Decisions (Architecture)",
      "## 2. Implementation-Level Decisions (Zero-Decision Rust)",
      "## 3. Pattern Selection",
      "## 4. Anti-Patterns",
      "## 5. Default Architecture",
      "## 6. Default Stack",
      "## 7. Severity Guide",
  ):
    assert heading in text, f"Missing {heading} in {STANDARD}"


def test_rust_persona_reads_the_standard() -> None:
  persona = read("agents/rust.md")
  assert "name: egm-rust-reviewer" in persona
  assert f"`{STANDARD}`" in persona
  assert "Mode: BRAINSTORM" in persona and "Mode: REVIEW" in persona


def test_drafting_records_language_and_rust_decisions() -> None:
  instructions = read("resources/execution_instructions.md")
  assert "**Language Default (Rust)**" in instructions
  assert "`.rs`" in instructions and "`Cargo.toml`" in instructions
  template = read("skills/egm-draft/references/design_doc.md")
  assert "*   **Language:**" in template
  assert "### Rust Design Decisions" in template
  draft = read("skills/egm-draft/SKILL.md")
  assert "**Resolve Language**" in draft
  assert STANDARD in draft


def test_review_stages_dispatch_the_rust_reviewer() -> None:
  review = read("skills/egm-review/SKILL.md")
  assert review.count('"egm-rust-reviewer": "rust-standard"') == 2
  assert review.count("egm-rust-reviewer]") == 4
  loop = read("skills/egm-review-loop/SKILL.md")
  assert "`rust-standard`" in loop and "`rust`" in loop


def test_rust_standard_defines_layers_and_responsibilities() -> None:
  text = read(STANDARD)
  for crate in ("`domain`", "`application`", "`proto`", "`infrastructure`"):
    assert f"| {crate} |" in text, f"Missing {crate} row in {STANDARD}"
  assert "**Single Responsibility (Author Preference)**" in text
  for crate in ("tokio", "axum", "tonic", "tokio-postgres", "Tauri", "Dioxus",
                "validations_rs"):
    assert crate in text, f"Missing {crate} in the default stack"


def test_install_commands_point_to_the_fork() -> None:
  readme = read("README.md")
  assert "claude plugin marketplace add mdambro/elephant-goldfish-model" in readme
  assert "marketplace add google/elephant-goldfish-model" not in readme


def test_onboarding_material_is_present() -> None:
  for path in ("docs/onboarding/README.md",
               "docs/onboarding/Patrones_y_arquitectura_en_Rust.pdf",
               "docs/onboarding/induccion/Cargo.toml",
               "docs/onboarding/induccion/crates/proto/proto/users/v1/users.proto"):
    assert (ROOT / path).is_file(), f"Missing {path}"
  assert "AsyncValidationError::CheckFailed" in read(STANDARD)


def test_plugin_is_named_egm_rust_on_every_host() -> None:
  for path in ("plugin.json", ".claude-plugin/plugin.json",
               ".codex-plugin/plugin.json", ".cursor-plugin/plugin.json",
               ".plugin/plugin.json"):
    assert json.loads(read(path))["name"] == "egm-rust", path
  for path in (".claude-plugin/marketplace.json",
               ".agents/plugins/marketplace.json"):
    marketplace = json.loads(read(path))
    assert marketplace["name"] == "egm-rust", path
    assert marketplace["plugins"][0]["name"] == "egm-rust", path
  entry = read("com.github.copilot/agents/egm-rust.agent.md")
  assert "name: egm-rust\n" in entry.replace("\r\n", "\n")
  opencode = read("adapters/opencode/install.py")
  assert '"default_agent": "egm-rust"' in opencode


def test_shipped_text_has_no_upstream_plugin_ids() -> None:
  """A merge from upstream can reintroduce ids; only the translation page may name them."""
  legacy = re.compile(r"\begm:|\begm@|--agent egm\b(?!-)|plugins approve egm\b(?!-)")
  paths = [ROOT / "README.md"]
  for pattern in ("docs/*.md", "skills/*/SKILL.md", "agents/*.md",
                  ".claude-plugin/agents/*.md", ".cursor-plugin/agents/*.md",
                  "com.github.copilot/agents/*.md"):
    paths += ROOT.glob(pattern)
  for path in paths:
    if path.name == "upstream_docs.md":
      continue
    found = legacy.findall(path.read_text(encoding="utf-8"))
    assert not found, f"{path.relative_to(ROOT)}: {found}"


def test_upstream_translation_page_is_linked_from_the_readme() -> None:
  assert "docs/upstream_docs.md" in read("README.md")
  page = read("docs/upstream_docs.md")
  assert "`claude plugin install egm-rust@egm-rust`" in page
  assert "egm-rust-reviewer" in page


def test_fork_publishes_no_website() -> None:
  """egm.dev belongs to upstream; a merge must not bring back a deployment."""
  assert not (ROOT / "site").exists()
  assert not (ROOT / ".github/workflows/deploy-site.yml").exists()


def test_new_issues_go_to_the_fork() -> None:
  upstream = "github.com/google/elephant-goldfish-model/issues/new"
  for path in ("ACCESSIBILITY.md", "AI_POLICY.md", "CONTRIBUTING.md"):
    assert upstream not in read(path), path
  assert json.loads(read(".claude-plugin/plugin.json"))["supportUrl"].endswith(
      "mdambro/elephant-goldfish-model/issues")
