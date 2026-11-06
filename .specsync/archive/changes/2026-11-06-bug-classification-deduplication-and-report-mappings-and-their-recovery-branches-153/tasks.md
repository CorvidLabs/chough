---
change: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153
artifact: tasks
---

# Tasks

- [x] Add the six REQ-cli-054 tests in tests/bugs.rs. The duplicate-reopen and link-recovery tests fail first.
- [x] Change src/bugs.rs as the design says: reopening clears `duplicate_of`, self-related links are refused, and the refusals name their recovery commands.
- [x] Add REQ-cli-054, the docs/cli.md and FRD lines, and a CHANGELOG line.

After `specsync change check` materializes the delta, not a task box:
- record the failing-first observations, link them to a local bug, then record the passing ones and verify that bug (retained failure and fix proof);
- record the mutation logs;
- run `fledge lanes run verify` on Linux.
