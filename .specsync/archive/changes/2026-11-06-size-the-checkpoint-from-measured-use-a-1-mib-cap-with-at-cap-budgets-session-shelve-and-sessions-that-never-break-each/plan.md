---
change: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each
artifact: plan
---

# Plan

## Approach

Change the cap, add the recovery, and make reads per-session, all behind failing tests.

## Out of scope

- KNOW-29, keep-five (its own change).
- Compacting a session's content.
- The status report's own size (it is the complete report).

## Steps

1. Leif approves the scope.
2. Commit the failing tests:
   - the cap constant in cli and stages;
   - the at-cap budget and one-over test, and the full stress shape, in next;
   - per-session robustness and shelve, in both modes, in sessions.
3. Implement the cap, the refusal text, per-session reads, shelve, and retention over `shelved/`.
4. Update docs/cli.md, docs/architecture.md, the FRD and the CHANGELOG.
5. Run verify on Linux, and a macOS spot check.

## Risks

See design.

## Constraints consulted

DRIVE-14; Leif's 1 MiB and "Add shelve" (2026-11-05).
