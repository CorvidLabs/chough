---
change: a-blocking-bug-stops-only-a-session-it-is-related-to
artifact: design
---

# Design

Both call sites read one split of the open blocking bugs. A bug stops the session when the session lists it, its evidence contains a link starting with that session's id, or it shares a criterion, requirement, or case with the session. Every other open blocking bug is named on chough next and does not stop the session. A bug that names nothing is in that second set.

The refusal text for a related bug stays refused:blocking-bugs. Skip and advance --to use the same forward step as advance.
