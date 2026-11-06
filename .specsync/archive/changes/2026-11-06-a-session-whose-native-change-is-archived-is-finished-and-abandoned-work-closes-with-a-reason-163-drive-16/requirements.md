---
change: a-session-whose-native-change-is-archived-is-finished-and-abandoned-work-closes-with-a-reason-163-drive-16
artifact: requirements
---

# Requirements

REQ-history-006 (MODIFIED, in `deltas/history.md`; the base text includes #155's amendment from #162):
- `session new` and `session use` archive every other finished session. A session is finished at done, or when its native change is archived on HEAD, read from `finalization.json` without a native read. The kept-open rule for unfolded inbox entries still applies.
- New: `chough session close <key> --reason <text>`, with its refusals, the `Closed without done: <reason>` decision, the no-replace write into the archive and the pointer clear.
- Two acceptance criteria are added, and DRIVE-16 joins the trace.

REQ-cli-037 (MODIFIED, in `deltas/cli.md`):
- With no pointer, the only open session, or else the only unfinished one, is active.
- A finished session does not count toward several open sessions, and no native read is needed to tell.
- One acceptance criterion is added, and DRIVE-16 joins the trace.

Every other sentence and criterion is unchanged.
