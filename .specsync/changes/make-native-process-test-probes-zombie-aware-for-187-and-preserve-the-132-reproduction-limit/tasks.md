---
change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
artifact: tasks
---

# Tasks

- [x] Add the existing `kill -0` helper and a deterministic zombie regression; record its failing run and commit before the fix.
- [x] Implement bounded zombie-aware `common::alive` and use it in the timing, native and change probes.
- [x] Add per-change cli and reporting cases and record failure-first and passing observations.
- [x] Run focused and loaded macOS regressions (`fledge run test`, full suite green); Linux covered by CI's Ubuntu jobs; `fledge run linux-verify` was attempted and stopped at the SDD audit while the change was still in the approved state (no product failure), to be rerun once verification is current; #132's reproduction limit stays open and unchanged.
- [x] Refresh generated knowledge and complete strict native verification.

Remaining after verification, as plain text rather than checkboxes (they are gates, not worker tasks): present the change for review, then human implementation review, archive approval and pull-request merge; any follow-up on #132 stays a separate decision.
