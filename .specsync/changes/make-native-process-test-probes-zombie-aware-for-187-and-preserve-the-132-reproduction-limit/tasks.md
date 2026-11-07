---
change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
artifact: tasks
---

# Tasks

- [ ] Add the existing `kill -0` helper and a deterministic zombie regression; record its failing run and commit before the fix.
- [ ] Implement bounded zombie-aware `common::alive` and use it in the timing, native and change probes.
- [ ] Add per-change cli and reporting cases and record failure-first and passing observations.
- [ ] Run focused and loaded macOS/Linux regressions plus `fledge run linux-verify`; record the results and #132's unchanged open limit.
- [ ] Refresh generated knowledge, complete strict native verification, and present the change for orc-mediated review.

Human implementation review, archive approval, pull-request merge and any follow-up on #132 are orc-mediated gates, not worker task checkboxes.
