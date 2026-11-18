---
change: fix-277
artifact: context
---

# Context

GitHub #277, found in the review of #256. Since #256 a migrated bug's front
matter names each carried observation by the name of the entry or receipt it
became. The bug's Markdown body, in its Log lines, still named them by bare
0.5.x ID, so a reader of the file could not follow them to the session or story
that holds the entry without searching.

Leif's decision, in his comment on #277 on 2026-11-18 from his planning
interview: "relative links. A migrated bug's body links each carried
observation to the file of the entry or story that holds it, as a new bug's
body does."

A new bug's Log names its evidence as `<session-id>/<evidence-id>`, which is not
a Markdown link either; this change follows the decision's first sentence and
writes relative Markdown links in a migrated bug's Log. A new bug's Log is not
changed.

No requirement text changes: REQ-records-009 already says each bug becomes a
Markdown bug and each carried observation an entry or a receipt.
