# Lesson bundle: fix-437

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 437
- **Kind**: Feature
- **Specs**: history, records, cli
- **Paths**: src/session_md.rs, src/direct_writes.rs, tests, changelog.d
- **Acceptance**: When a direct lifecycle write records a state of none (state.json absent), its Timeline entry shows before_state: none and after_state: none rather than omitting the lines, and the entry's text says state none, not unknown.

## Evidence

- Verification commit: `5e921dc7ac02daa398b3c2c84704ea01fb5e7048`
- Base commit: `35828a1df3d308677a38b138e7aebe0e7b6770ef`
- Verified by: `specsync check --spec cli --spec history --spec records --spec reporting`

## From the change's context.md

# Context

Found by the 0.6.2 documentation audit and filed as chough#437.
REQ-history-013 says a direct lifecycle write's `before_state` and
`after_state` are `none` when `state.json` is absent. The Timeline
writer leaves those lines out entirely when the state is absent
(src/session_md.rs, the `part` closure skips `None`, lines about 730),
and the entry's text says "unknown" (src/direct_writes.rs, the `state`
closure about line 116). docs/cli.md documents `none`.

## From the change's testing.md

# Testing

A direct-native-write entry with an absent state reads back with
before_state: none and after_state: none, and its text says `state stays
none`. The session/history tests covering direct-write entries cover it;
the change's own check lane plus fmt and lint.

## Where these lessons go

- `specs/history/context.md`
- `specs/records/context.md`
- `specs/cli/cli.3md`
