---
change: let-a-session-started-after-legacy-history-adoption-write-events-while-the-adopted-legacy-log-stays-guarded
artifact: testing
---

# Testing

tests/sessions.rs `a_session_started_after_legacy_adoption_writes_events_and_keeps_the_log_guarded`. A legacy project with a transition log runs `history adopt --apply`, `session adopt legacy` and `session new other`. In `other`:
- `stage advance` writes one event to its own cycle, and the legacy log keeps its bytes;
- its cursor still carries no legacy anchor, so history walks never read the legacy log as its past;
- after the log gains a byte, `stage advance` refuses with "legacy history anchor changed", in both sessions.

The test fails before the fix, on the first `stage advance`. `fledge lanes run verify` passes on Linux, and the adoption change's two-branch simulation then runs on this repository.
