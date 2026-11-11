# Lesson bundle: know-29-roll-up-copies-never-moves-tracked-session-records-165

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: KNOW-29 roll-up copies never moves tracked session records (#165)
- **Kind**: BugFix
- **Specs**: history
- **Paths**: src/sessions.rs, tests/reduce.rs
- **Acceptance**: At the move into done with the librarian on, archived sessions beyond the five newest are copied byte for byte into the vault (hard link sharing bytes) and remain in .chough/sessions/archive/ unchanged; the report names them under copied. A vault attachment with identical bytes means nothing to do; different bytes leave both sides untouched and are reported. The legacy-anchor holder never rolls. Tracked session records never leave the tree, whether or not knowledge/ is committed.

## Evidence

- Verification commit: `5427b09f421dba93f1fe23f5bc754684290e264f`
- Base commit: `5359f42550362eebac3eef28d4df34639ffeb9c2`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #165 (Codex's P2 on CorvidLabs/spec-sync#941, deferred here): the KNOW-29 roll-up moved archived session records into the vault; where `knowledge/` is untracked, tracked records left the tree. Leif's decision (2026-11-10, recorded by the coordinator on the issue): "copy, never move" — tracked session records stay put; the vault gets a copy or a link.

Consequence flagged for acceptance: a rolled session stays listed by `session list` and stays a retention reference source, since its record no longer leaves the archive directory.

## From the change's testing.md

# Testing

| Requirement | Coverage |
|---|---|
| REQ-history-011 | Updated roll-up tests in tests/reduce.rs: rolled sessions stay in `.chough/sessions/archive/` byte-identical and the vault holds copies; anchor-holder and librarian-off behavior unchanged |

tests/reduce.rs (43), tests/sessions.rs (30) and tests/evidence.rs (36) pass with the change.

## Where these lessons go

- `specs/history/context.md`
