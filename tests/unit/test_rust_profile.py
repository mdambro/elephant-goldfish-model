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
      "## 5. Default Architecture",
      "## 6. Default Stack",
      "## 7. Severity Guide",
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
