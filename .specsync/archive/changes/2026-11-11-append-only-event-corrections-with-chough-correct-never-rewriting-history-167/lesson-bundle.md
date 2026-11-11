# Lesson bundle: append-only-event-corrections-with-chough-correct-never-rewriting-history-167

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Append-only event corrections with chough correct, never rewriting history (#167)
- **Kind**: Feature
- **Specs**: history, reporting, cli
- **Paths**: src/main.rs, src/stage.rs, src/history.rs, src/checkpoint.rs, src/timing.rs, tests/history.rs, docs/cli.md, README.md, tests/spec_split.rs
- **Acceptance**: chough correct <event> --reason appends a correction event naming its EVT- target with the corrected statement in the active session's current cycle; the target's bytes are unchanged; unknown ids, blank or oversized statements and correcting a correction are refused with nothing written; next and status show the correction with its target; timing treats the kind as known with no stage segment.

## Evidence

- Verification commit: `364eb89885ddc882423217d8d4bdb0ca8cb2e0c6`
- Base commit: `00c34d65d3eb9c56b8f24ed471fcc4e71ebbd90f`
- Verified by: `specsync check --spec cli --spec history --spec reporting`

## From the change's context.md

# Context

Issue #167 (Codex's P2 on CorvidLabs/spec-sync#941, deferred here by Leif): a wrong `stage reset --reason` was recorded verbatim, and once the session archived, nothing in `.chough/` could say so without hand-editing. History events are immutable (REQ-history-002), so the correction must be a new append-only record that points at the event it corrects.

Open questions for the human: the command shape (for example `chough history correct <event-id> --reason ... --statement ...`), whether corrections also apply to decisions/open questions in open sessions or only to history events and archived sessions, and whether readers show corrections by default or behind a flag.

## From the change's design.md

# Design

No visual or interaction surface changes: one new command and a corrections listing in next/status text and JSON. No prototype.

## From the change's testing.md

# Testing

Done: RED regressions for appending a correction to a recorded event, surfacing it in history show/next/status and for an archived session's next_action, and proof that the original event bytes and digests are unchanged.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-015 (added) | tests/history.rs (`a_correction_appends_and_is_shown_without_rewriting_history`) |
| REQ-cli-021 (modified) | tests/history.rs (the same test's next/status assertions) |
| REQ-reporting-001 (modified) | tests/timing.rs (existing kind-handling coverage; the correction kind joins the no-segment set) |

## Where these lessons go

- `specs/history/context.md`
- `specs/reporting/context.md`
- `specs/cli/cli.3md`
