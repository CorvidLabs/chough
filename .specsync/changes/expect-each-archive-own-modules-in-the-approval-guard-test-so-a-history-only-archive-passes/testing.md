---
change: expect-each-archive-own-modules-in-the-approval-guard-test-so-a-history-only-archive-passes
artifact: testing
---

# Testing

On this branch, before the change,
`cargo test --locked --test stages the_guard_accepts_every_archived_approval_in_this_repository`
fails with "remove-the-history-events-…: unchanged: history", as hosted CI did on #96.
After the change it passes. `fledge lanes run verify` passes on Linux.
