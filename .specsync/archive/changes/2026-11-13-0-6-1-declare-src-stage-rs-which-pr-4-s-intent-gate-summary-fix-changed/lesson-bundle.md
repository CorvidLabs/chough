# Lesson bundle: 0-6-1-declare-src-stage-rs-which-pr-4-s-intent-gate-summary-fix-changed

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: 0.6.1: declare src/stage.rs, which PR 4's intent gate summary fix changed
- **Kind**: Refactor
- **Specs**: cli
- **Paths**: src/stage.rs
- **Acceptance**: src/stage.rs, changed by PR 4 (its added_and_modified helper split for the intent gate summary), is covered by a change, so specsync change audit finds no uncovered path; no code or contract changes

## Evidence

- Verification commit: `80ead7c3a7d05cf5925cb69d572605e97d903ec2`
- Base commit: `30b770aafc0b5ecce8185df9de30cd2594a0519f`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

PR #242 (0.6.1 PR 4, the intent gate summary) shipped its change archived, but hosted
CI's `specsync change audit` refused: "meaningful changed paths are not covered by an
active change: src/stage.rs". The fix split `added_or_modified` in `src/stage.rs` into
`added_and_modified`, which `src/gates.rs` uses, and the change's file list named only
`src/gates.rs`, `tests/gates.rs`, the docs and the CHANGELOG. An archived change's paths
cannot be amended. On 2026-11-13T02:53Z Leif chose "Yes, fix and keep (Recommended)":
one yes covers this record's scope and review.

## From the change's testing.md

# Testing

`specsync change audit` in the worktree reports no uncovered path after this change.
PR 4's cases and the full suite cover the code itself.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-061 | `tests/gates.rs`: PR 4's cases, which use the `src/stage.rs` helper |

## Where these lessons go

- `specs/cli/cli.3md`
