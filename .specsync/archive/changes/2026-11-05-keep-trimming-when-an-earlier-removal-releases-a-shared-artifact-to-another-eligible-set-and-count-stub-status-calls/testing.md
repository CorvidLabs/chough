---
change: keep-trimming-when-an-earlier-removal-releases-a-shared-artifact-to-another-eligible-set-and-count-stub-status-calls
artifact: testing
---

# Testing

tests/reduce.rs `a_shared_artifact_released_by_an_earlier_removal_does_not_stop_the_trim` gives two superseded runs listed by a cold archive one shared log and their own logs. Both must be trimmed with their own logs, and the shared log kept. It failed before the fix with the same refusal as the repository ("r2: its files changed"). The existing protection-change refusals in tests/evidence.rs keep passing. The trim then runs on this repository at closeout, and `fledge lanes run verify` runs on Linux.
