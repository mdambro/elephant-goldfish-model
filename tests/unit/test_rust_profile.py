# SPDX-License-Identifier: Apache-2.0

"""Keep the Rust default profile wired into drafting and review."""

from pathlib import Path

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
      "## 5. Severity Guide",
  ):
    assert heading in text, f"Missing {heading} in {STANDARD}"


def test_rust_persona_reads_the_standard() -> None:
  persona = read("agents/rust.md")
  assert "name: egm-rust" in persona
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
  assert review.count('"egm-rust": "rust-standard"') == 2
  assert review.count("egm-rust]") == 4
  loop = read("skills/egm-review-loop/SKILL.md")
  assert "`rust-standard`" in loop and "`rust`" in loop
