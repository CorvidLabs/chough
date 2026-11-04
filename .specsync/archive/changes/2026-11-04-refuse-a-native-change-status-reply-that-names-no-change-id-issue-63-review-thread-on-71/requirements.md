---
change: refuse-a-native-change-status-reply-that-names-no-change-id-issue-63-review-thread-on-71
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-007 (see `deltas/cli.md`). With SpecSync configured, a
`specsync change status <id> --json` reply that is JSON but names neither `summary.id`
nor `change.id`, such as `[]` or `{}`, no longer saves: `checkpoint --native-change`
refuses it as `refused:native-change-unknown` and `.chough/session.json` stays
byte-identical. The rest of REQ-cli-007 is unchanged.
