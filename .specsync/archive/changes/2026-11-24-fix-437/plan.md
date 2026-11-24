---
change: fix-437
artifact: plan
---

# Plan

## Approach

1. In src/session_md.rs, where the direct-native-write entry is written,
   pass `Some(write.before_state.as_deref().unwrap_or("none"))` and the
   same for after_state to `part`, so the lines always appear.
2. In src/direct_writes.rs `text`, change the `state` closure's fallback
   from "unknown" to "none".
3. Update or add the tests that assert on a direct-write entry's fields
   and text when the state is absent.
4. Changelog fragment changelog.d/2026-11-23-fix-437.md.

## Out of scope

Other Timeline fields, the reader's parsing beyond what the tests need,
and any contract text.

## Steps

As above, then the session/history tests, fmt, lint, the change's check
lane.

## Risks

A test may assert the lines are absent when the state is missing; the
same commit updates it. The reader must accept the value `none`; the
tests cover reading back.

## Constraints consulted

REQ-history-013 and docs/cli.md; the existing `part`/`written_part`
conventions.