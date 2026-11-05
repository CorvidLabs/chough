---
change: lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/98, found in an outside review of #77
and reproduced at 75a732d:

1. `inbox add --session alpha` checks that alpha is open and then writes its entry with
   no lock, so a concurrent `session use beta` can archive a done alpha in between; the
   entry is stranded (4 of 300 unpaused runs; deterministic with a pause).
2. Retention does not count observation IDs named by unfolded inbox entries, so
   `evidence prune --apply` can remove an observation a subagent has reported, and the
   later fold adds a missing ID.

Leif asked on 2026-11-04 for a PR stacked on #77 (the "Open a PR on #77" option).
Criteria: DRIVE-3 and LIFE-8. This branch is stacked on #77 at 75a732d.

The fix stays in history-owned code: retention gathers checkpoint references through
`sessions::every_checkpoint`, its only caller, so that function also returns each
session's unfolded inbox observations, labelled `inbox of session <key>`. src/evidence.rs
(cli-owned) is unchanged.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes and his
answer above.
