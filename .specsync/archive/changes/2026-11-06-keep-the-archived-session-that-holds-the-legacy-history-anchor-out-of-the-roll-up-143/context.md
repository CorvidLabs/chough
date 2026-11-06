---
change: keep-the-archived-session-that-holds-the-legacy-history-anchor-out-of-the-roll-up-143
artifact: context
---

# Context

Issue #143, a follow-up to #141. Leif chose option A on 2026-11-06 and amended KNOW-29 in his words ("Amend KNOW-29"), with this wording: "The session that adopted the legacy history (it holds the legacy anchor) is never rolled into the vault; it stays in .chough/sessions/archive in addition to the five, so history never depends on the knowledge directory."

The #141 fix lets `adopted_anchor` find the legacy anchor in sessions rolled into the vault. So today the anchor depends on the knowledge directory: its `library-dir` preference and its files. The KNOW-29 roll-up (REQ-history-011) can roll the session holding the anchor once five newer sessions are archived.

The KNOW-29 test fixture copies the adopting session, cursor included, into each of the seven archived sessions it fabricates, so all eight hold the anchor. In a real project only the adopting session holds it.
