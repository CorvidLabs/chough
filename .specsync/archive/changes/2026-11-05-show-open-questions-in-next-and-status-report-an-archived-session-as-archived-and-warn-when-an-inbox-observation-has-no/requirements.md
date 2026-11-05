---
change: show-open-questions-in-next-and-status-report-an-archived-session-as-archived-and-warn-when-an-inbox-observation-has-no
artifact: requirements
---

# Requirements

MODIFIED REQ-cli-021 (see `deltas/cli.md`): `next` shows the checkpoint's open
questions, bounded like blockers, and `status` reports them. MODIFIED REQ-history-006
(see `deltas/history.md`): `session list` reports an archived session's status as
`archived`, and `inbox add` warns about an observation entry with no record, which `inbox list`
marks `recorded: false`.
