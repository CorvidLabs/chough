---
change: fix-347
artifact: context
---

# Context

GitHub #347, two follow-ups from the tester's check of #341 (#277). Leif chose
both.

1. A new bug's Log named its evidence as `<session>/<evidence>` with no link,
   while a migrated bug's Log links it since #341. Every bug should read the
   same.
2. A link to an open session's evidence entry goes stale when that session is
   archived: the folder is removed and the entry lives on as a receipt of the
   story. The issue allows either repointing at archive time or having
   `chough doctor` report and fix stale links, and asks for the smaller honest
   one.

For 2 this change takes the doctor route. A bug's Log is append-only
(REQ-cli-012), so an earlier line's link cannot be rewritten. `chough doctor`
reports the bug as `stale_bug_links`, and `--fix` appends one Log entry that
links the archived story. Archiving itself is unchanged: it still touches no
record but the session's own. The cost is that the link is stale until doctor
runs.

The front matter keeps the stored `<session>/<id>` form, which already resolves
to the story's receipt after archiving.
