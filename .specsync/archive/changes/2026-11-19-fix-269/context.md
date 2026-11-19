---
change: fix-269
artifact: context
---

# Context

GitHub #269, found by a fresh-eyes UX run on 0.6.0. `chough session new` without an objective writes the placeholder "Describe the current objective before beginning work." as the story's `# <title>` and its Goal. `chough checkpoint --objective` then updates the front matter only. The session record, its archived story, and the journal post built from the Goal all keep the stale placeholder, text that was never the objective.

Leif's decision (2026-11-18, planning interview, recorded on #269 and relayed by hc t4c as the scope approval) is to fill the placeholder once. When the session's Goal section still holds the untouched default text, `checkpoint --objective` replaces it. A Goal a person has written is never rewritten. The journal post and the archived story use the current objective. This adds one exception to REQ-records-004.

Asked whether the `# <title>` line, which holds the same placeholder, is filled too, he answered "Yes, fill both" (relayed by hc t4c, 2026-11-18).
