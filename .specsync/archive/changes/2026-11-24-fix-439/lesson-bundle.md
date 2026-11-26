# Lesson bundle: fix-439

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: fix-439
- **Kind**: Feature
- **Specs**: cli
- **Paths**: docs/cli.md, src/change.rs, src/checkpoint.rs, src/gates.rs, src/native_check.rs, tests/humans.rs, tests/native_check.rs, tests/relay.rs, .chough/cases/cli.fix-439.cases.toml
- **Acceptance**: next and status surface the refusal recorded in verification-attempts.json instead of the keeps-no-reason note, with the fallback note unchanged when no refusal is recorded, REQ-cli-039 amended by delta to match, docs/cli.md updated, full suite green

## Evidence

- Verification commit: `339cdd6e1f9d26c922a8ab7c0b5c8d980a77cf1c`
- Base commit: `bdda89e40b5ff3086e3d571305f82215a120f8a5`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context — fix-439

## Problem
`chough next` and `chough status` render a note that "SpecSync keeps no
reason" when the native check has not passed, telling the user to run
`specsync change check` to see why. But since fix-437's era the refusal
reason *is* recorded: `.specsync/changes/<id>/verification-attempts.json`
writes a `"passed": false` attempt with a `"refusal"` field (and docs/
cli.md documents the field). The CLI just never reads it back.

## Evidence
- `src/native_check.rs` ~line 62: the `(Some("approved" | "implementing"), _)`
  arm emits the "SpecSync keeps no reason when it refuses before
  verifying" note without consulting the attempts file.
- `docs/cli.md`: documents that `verification-attempts.json` records
  `"refusal"` for pre-verification refusals, while `next`/`status` say
  "not shown yet" in the same table row.

## Affected requirement
REQ-cli-040 (native readiness surfaced in `next`/`status`) — held by
the `cli` spec. No contract change: the note and the recorded reason
describe the same refusal; this change only makes the CLI prefer the
recorded reason when one exists.

## Constraints consulted
- `chough change check` refuses while tasks.md has unchecked boxes
  (vault lesson, 2026-10-31) — testing.md keeps plain text, no boxes.
- The attempts file is the same one `change check` appends to; reading
  it must never write it.
- docs test pins `.gitattributes` merge-driver lines exactly — do not
  touch `.gitattributes`.
## Re-scope correction (2026-11-24)
The change was created with `--no-spec-change`, but the GLM second
review correctly found the rendered-note change alters what REQ-cli-039
SHALL says. Leif authorized adding the delta ("Yes", pi chat t14z).
`specsync change send-back` put the change back to draft; the interview
was re-answered with `public_contract yes`; the flag correction in
`state.json` records what the interview would have said (no requirement
text was hand-edited — the amendment lives in `deltas/cli.md`).

## From the change's testing.md

# Testing — fix-439

Planned coverage (to be recorded after verification):

1. Unit/integration test: a change directory with
   `verification-attempts.json` whose latest attempt has
   `"passed": false` and `"refusal": "..."` — `next`/`status` output
   contains the refusal text and no longer contains "SpecSync keeps no
   reason".
2. Test: no attempts file (or latest attempt passed) — output keeps the
   existing fallback note.
3. Full suite: `cargo test --locked` green (all suites).

Verification environment: local-macos-arm64.

## Where these lessons go

- `specs/cli/cli.3md`
