---
change: fix-250
artifact: context
---

# Context

GitHub #250, found on a CorvidLabs/spec-sync clone after its migration.
`chough doctor --fix --reason "<text>"` withdrew five stale gates and archived
their sessions. The reason, which doctor requires before it applies a decision
fix, was written nowhere: not on the `gate-closed` Timeline entries, not in the
stories, and not in doctor's JSON. A reader of the archive could not tell why a
human gate was withdrawn.

Leif's decision, in his comment on #250 on 2026-11-18 from his planning
interview: "record it on each gate. When `doctor --fix --reason` withdraws stale
gates, each withdrawn gate's Timeline entry and the archived story carry the
reason in the person's words, and doctor's JSON echoes it back. REQ-history-012
gets that sentence."

The reason is a new field, `withdrawn`, on the gate's `gate-closed` entry. The
entry's own text stays the gate's text, which other readers match on, and
withdrawing still records no human decision. The story ends with the Timeline,
so it carries the reason through that entry.
