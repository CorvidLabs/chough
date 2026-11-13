# Lesson bundle: 0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: 0.6.1: chough next with no open session says so and exits 0, and timing says elapsed wall-clock once
- **Kind**: BugFix
- **Specs**: cli, history, reporting
- **Paths**: CHANGELOG.md, docs/cli.md, src/main.rs, src/sessions.rs, src/timing.rs, tests/common/mod.rs, tests/next.rs, tests/reduce.rs, tests/sessions.rs, tests/spec_split.rs, tests/timing.rs
- **Acceptance**: With no open session, chough next prints that there is no open session and names chough session new <key>, exiting 0, while chough status still refuses with that message; chough timing says elapsed wall-clock once, in its header, on no gate-wait, segment or estimate line, and its notes say read-only once (red then green)

## Evidence

- Verification commit: `7363d8139c697930a4f05c4c9fc47983c07400fe`
- Base commit: `9d53604071df50b537adccad0ccf5cac9a2fc32b`
- Verified by: `specsync check --spec cli --spec history --spec reporting`

## From the change's context.md

# Context

Chough 0.6.0's fourth fresh-eyes round (t38n) found two output items, deferred to 0.6.1
by Leif. F10: `chough next` with no open session, the normal state after the move into
done, prints "Error: no open session" and exits 1. F5: `chough timing` still prints
"(elapsed wall-clock)" on the human-gate line, though it should say so once in its
header, and a note reads "read read-only". On 2026-11-13T02:03Z Leif chose "Yes, start
(Recommended)" for 0.6.1 as six small PRs; this is one. Bugs:
`.chough/bugs/2026-11-12-chough-next-with-no-open-session-exits-1-with-error-no-open.md`
and `.chough/bugs/2026-11-12-chough-timing-still-says-elapsed-wall-clock-on-the-human-gate.md`.

## From the change's design.md

# Design

`chough next` with no open session prints the no-session message naming `chough session
new <key>` and exits 0; `chough status` keeps refusing, since a full report needs a
session. `chough timing`'s text drops the label from its gate-wait line, keeping it once in
the header, and the native-record note reads read-only once.

## From the change's testing.md

# Testing

`tests/next.rs` or `tests/sessions.rs`: after the move into done, `chough next` prints the
no-session message naming `chough session new <key>` and exits 0, while `chough status`
refuses. `tests/timing.rs`: the text says elapsed wall-clock once, in its header, and no
note says "read read-only". Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-021 | `tests/next.rs`: with no open session, next exits 0 naming `chough session new <key>` |
| REQ-history-006 | `tests/sessions.rs`: after the move into done, next exits 0 and status refuses |
| REQ-reporting-001 | `tests/timing.rs`: elapsed wall-clock is said once, in the header |

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
- `specs/reporting/context.md`
