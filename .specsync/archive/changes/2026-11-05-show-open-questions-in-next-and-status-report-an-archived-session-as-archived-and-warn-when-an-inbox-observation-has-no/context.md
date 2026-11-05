---
change: show-open-questions-in-next-and-status-report-an-archived-session-as-archived-and-warn-when-an-inbox-observation-has-no
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/84 comes from a field test on
2026-11-03 against #77 head 2c53f44 (SpecSync 6.1.2):

- A subagent's `inbox add --kind question` folds into the session's
  `open_questions`, but `next`, `next --json` and `status` never show open questions,
  so the question is silently parked. The same holds for `checkpoint --question` on
  main: the checkpoint stores open questions and only `context` shows them.
- a. `session list` shows an archived session as `"archived": true, "stage": "done",
  "status": "active"`.
- b. `inbox add --kind observation --text OBS-bogus` exits 0 and folds; only a later
  `next` says the ID is unavailable.

This branch is stacked on #77 (`corvid-agent/feat/sessions-per-objective` at 290cce3),
because items a and b live in its sessions code; it retargets to main when #77 merges.

Criteria: LIFE-3 ("I can resume work with focused context, decisions, open questions,
and the next action") and DRIVE-3 already cover this; no new HI wording is needed.

Item b: on 2026-11-04 Leif chose "Warn, don't refuse" from offered options: `inbox
add` still adds an observation entry whose ID has no record, warns on stderr, and
`inbox list` marks it, because subagents may report before the parent tree has the
record and `checkpoint --observation` accepts unknown IDs too. Refusing was offered
and not chosen.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes, given in
a Claude Code session, and his brief the same morning to fix issues 81 to 88 one PR
each.
