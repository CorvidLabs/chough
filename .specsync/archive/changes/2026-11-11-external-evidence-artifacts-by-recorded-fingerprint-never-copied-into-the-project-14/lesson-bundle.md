# Lesson bundle: external-evidence-artifacts-by-recorded-fingerprint-never-copied-into-the-project-14

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: External evidence artifacts by recorded fingerprint, never copied into the project (#14)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/evidence.rs, docs/cli.md
- **Acceptance**: evidence add and cases record accept an absolute artifact path: the file is fingerprinted (SHA-256) in place within a 64 MiB bound and never copied into the project; missing, symlinked, directory and oversized external paths are refused by name. Project-relative canonical artifact behavior, freshness re-reads, digests and record shape are unchanged; a later change to the external file flips the observation's artifact freshness.

## Evidence

- Verification commit: `538473bfe44ef617e7f851e19e52aa72e18cc637`
- Base commit: `9f0a2df721821504c0fdae3d43c911c70a63e661`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #14: items 1, 2 and 4 (env on bindings, colour-safe matching, clearer binding/bug-field errors) shipped in #16; the Codex audit on the issue confirms that. Item 3 remains: artifacts had to be copied into the project and committed. Leif's decision (2026-11-10, recorded by the coordinator): allow external evidence artifacts by recorded fingerprint instead of copying them into `.chough/`.

Choice made under that decision: absolute paths only (no `~` expansion), fingerprinted in place with a 64 MiB bound; symlinks, directories and missing files refused.

## From the change's testing.md

# Testing

| Requirement | Coverage |
|---|---|
| REQ-cli-006 | tests/evidence.rs `an_external_artifact_is_fingerprinted_in_place`: absolute path fingerprinted, nothing copied into the project, missing external path refused by name |

RED observed with the src change stashed (canonical-spelling refusal); GREEN with it.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-006 (modified) | tests/evidence.rs (`an_external_artifact_is_fingerprinted_in_place`) |

## Where these lessons go

- `specs/cli/cli.3md`
