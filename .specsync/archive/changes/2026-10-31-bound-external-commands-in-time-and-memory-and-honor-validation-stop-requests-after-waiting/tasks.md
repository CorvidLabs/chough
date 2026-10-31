---
change: bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting
artifact: tasks
---

# Tasks

- [x] Add the bounded runner and platform tree stop in src/records.rs with reader unit tests.
- [x] Run case bindings through it, with per-stream truncation notes and limitations.
- [x] Run native reads through it, with the deadline override and timed-out state.
- [x] Check stop and pause before every validation attempt and slice the interval.
- [x] Add integration tests for descendants, deadlines, output bounds and stop timing.
- [x] Add case definitions for BUG-0009 to BUG-0011 and record passing observations.
- [x] Update docs/cli.md.

Human gates and post-verification steps (not tasks): scope approval of this change;
push and CI on all three platforms; the human implementation review; native review and ship.
