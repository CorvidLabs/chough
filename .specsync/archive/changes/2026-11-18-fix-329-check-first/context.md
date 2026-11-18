---
change: fix-329-check-first
artifact: context
---

# Context

A follow-up to fix-329 (#343), from Leif's P2 review comment on #343: with fix-329, `chough bug add` while a session is active always joins the new bug to that session, but it does so only after `create` has written the bug record. With the active session already holding its 200 bug links, a new `bug add` wrote a 201st `.chough/bugs` record, then failed the join with "too many bug (limit 200)": the session stayed byte-identical and the new bug was left unlinked. That breaks REQ-cli-012's auto-link and leaves a failed command partly applied.

The standing rule Leif gave for 2026-11-18, "Yes, for today", covers a real gap his review finds in an already-approved PR: a small follow-up change inside the approved scope, a tester's glance, then hc t4c resolves the thread. hc t4c added the design point: check first and refuse with nothing written, as in #312, #340 and #332, rather than removing the file afterwards (#306's lesson).
