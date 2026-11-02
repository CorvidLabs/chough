---
change: unlinked-failure-ignores-not-run-leaving-test-keeps-refusing-when-a-referenced-case-s-latest-executed-observation
artifact: design
---

# Design

In `unlinked_failure` (src/stage.rs), filter the case's observations to outcome passed or failed before `max_by`. The ordering (recorded time, then ID), the linked-bug check and the refusal text are unchanged, and `assess` shares the same function, so next and status follow automatically. A case whose observations are all not-run yields no latest executed observation and does not refuse, as before.
