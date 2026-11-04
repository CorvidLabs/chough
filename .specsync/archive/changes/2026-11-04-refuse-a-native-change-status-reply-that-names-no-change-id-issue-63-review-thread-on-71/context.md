---
change: refuse-a-native-change-status-reply-that-names-no-change-id-issue-63-review-thread-on-71
artifact: context
---

# Context

Review thread on #71 (P2, src/checkpoint.rs, 2026-11-04 01:17 UTC): a successful
`specsync change status` reply only proves exit 0 with parseable JSON. Stand-ins that
returned `[]`, `{}` or another change's summary all saved `made-up`. e3ff647 refuses a
reply naming another change (`refused:native-change-mismatch`), but a reply that names
no change ID still saved; the Chough bot's review 5762088554 noted that as well. The
thread asks for the reported change ID to be required, with missing-ID and wrong-ID
regression tests.

The earlier change for #63 is archived on this branch, so this follow-up is its own
native change on the same pull request. SpecSync 6.1.2 always names `summary.id` and
`change.id`, so real replies are unaffected.

Ruled out: changing stage gates, `next` or `status` native reads.
