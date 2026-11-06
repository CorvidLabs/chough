---
change: stop-tests-stall-on-a-per-fixture-token-so-concurrent-runs-never-take-or-kill-each-other-processes-144
artifact: tasks
---

# Tasks

- [x] Add the failing decoy test for the backfill stop test.
- [x] Stall every stop test on `identity(<n>)`, a unique per-fixture token, and match and reap only that token: the 611, 621, 631, 641, 651, 661 and 671 families. The `sleep 671` `pkill` calls become `Reap` on the token.
- [x] Run the stop suite, and verify on Linux.
