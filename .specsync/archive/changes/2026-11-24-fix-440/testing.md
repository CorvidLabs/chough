---
change: fix-440
artifact: testing
---

# Testing

The refusal from `chough stage advance --to <not-ahead>` must contain
`--reason`. The test that asserts on this message is updated to expect the
new wording; the rest of the stage test suite guards the other hints.
Verification commands: the change's own lane (`specsync check --spec cli
--spec history --spec records --spec reporting`) plus the touched cargo
tests.