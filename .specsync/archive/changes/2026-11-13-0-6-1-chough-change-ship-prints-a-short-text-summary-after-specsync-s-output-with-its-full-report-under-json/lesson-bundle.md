# Lesson bundle: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: 0.6.1: chough change ship prints a short text summary after SpecSync's output, with its full report under --json
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/change.rs, tests/change.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: chough change ship prints SpecSync output, then a short text summary of the review, the ship, the gate close and the move into done, at most 12 lines; with --json it prints every step full report as before (red then green)

## Evidence

- Verification commit: `595d09309e6644f46c0608e8d91bab5ff9db6e60`
- Base commit: `9d53604071df50b537adccad0ccf5cac9a2fc32b`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

In Chough 0.6.0's fourth fresh-eyes round (t38n, F6), `chough change ship` printed
SpecSync's text, then a long JSON checkpoint and relayed decision, then the move
summary, all interleaved. Stage moves already print a short summary with their full
report under `--json` (REQ-cli-008). The bug is recorded as
`.chough/bugs/2026-11-12-chough-change-ship-prints-specsync-s-text-a-long-json.md`,
deferred to 0.6.1 by Leif. On 2026-11-13T02:03Z he chose "Yes, start (Recommended)" for
0.6.1 as six small PRs; this is one.

## From the change's testing.md

# Testing

`tests/change.rs`: a ship prints SpecSync's output first, then at most 12 lines naming
the review, the ship, the gate close and the move into done; with `--json` it prints
every step's full report as before. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-059 | `tests/change.rs`: change ship prints a short summary after SpecSync's output, and every report under `--json` |

## Where these lessons go

- `specs/cli/cli.3md`
