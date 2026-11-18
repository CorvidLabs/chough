---
change: a-blocking-bug-stops-only-a-session-it-is-related-to
artifact: tasks
---

# Tasks

Scope approval is the human gate. Implementation review, ship, and a pull request wait until asked.

Verification is the focused cargo tests named in testing.md, then fledge run fmt, fledge run test, fledge run lint, and fledge lanes run verify before review.
