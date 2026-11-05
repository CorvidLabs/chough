---
change: expect-each-archive-own-modules-in-the-approval-guard-test-so-a-history-only-archive-passes
artifact: context
---

# Context

`the_guard_accepts_every_archived_approval_in_this_repository` in tests/stages.rs
replays every archived approval ledger in this repository through `chough next`. It
asserted that `approved_deltas` starts with `unchanged: cli`, because every archive
until now had a cli delta. #96's change touches only the history module, so once it was
archived the test read `unchanged: history`, and hosted CI on #96 failed. #99's
history-only change would fail the same way.

The assertion now requires `unchanged:` followed only by modules in that archive's
own approved scope, or exactly `no semantic deltas (no_spec_change)`. That is stricter
per archive than before, and no module outside the approved scope is accepted. Leif
(0xLeif) said "Yes, record and merge" for PRs #90–#97 and #99 on 2026-11-04.
