# Lesson bundle: let-a-session-started-after-legacy-history-adoption-write-events-while-the-adopted-legacy-log-stays-guarded

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Let a session started after legacy history adoption write events while the adopted legacy log stays guarded
- **Kind**: BugFix
- **Specs**: history
- **Paths**: src/history.rs, tests/sessions.rs
- **Acceptance**: In a project that adopted a legacy transition log, a session started with chough session new writes events to its own cycle; the legacy log keeps its bytes, stays out of the new session's history, and a change to it still refuses writes in every session. The new test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `93922cf12df3bb17c81ff9290cb9065b24c18b7e`
- Base commit: `88466daafc72145962cade5c7406a9fa84fe570f`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

Adopting sessions on this repository (DRIVE-12) went in this order: `history adopt --apply`, then `session adopt`, then `session new <key>` for the next piece of work. In the new session, `stage reset` failed with "legacy history anchor changed; restore the recorded legacy bytes before writing events".

The cause: `session new` gives the new session a cursor with no legacy anchor (`legacy: null`). Every event write compares that against the anchor of the legacy log that is still present, so the comparison always fails. The adopted session itself is fine, because it holds the anchor and `stage reset` carries it into each new cycle.

So in any project that adopted a legacy log, every session started afterwards fails on its first transition. REQ-history-006's acceptance criterion, that two branches advancing different sessions merge without conflict, was tested only in projects that never had a legacy log.

## From the change's testing.md

# Testing

tests/sessions.rs `a_session_started_after_legacy_adoption_writes_events_and_keeps_the_log_guarded`. A legacy project with a transition log runs `history adopt --apply`, `session adopt legacy` and `session new other`. In `other`:
- `stage advance` writes one event to its own cycle, and the legacy log keeps its bytes;
- its cursor still carries no legacy anchor, so history walks never read the legacy log as its past;
- after the log gains a byte, `stage advance` refuses with "legacy history anchor changed", in both sessions.

The test fails before the fix, on the first `stage advance`. `fledge lanes run verify` passes on Linux, and the adoption change's two-branch simulation then runs on this repository.

## Where these lessons go

- `specs/history/context.md`
