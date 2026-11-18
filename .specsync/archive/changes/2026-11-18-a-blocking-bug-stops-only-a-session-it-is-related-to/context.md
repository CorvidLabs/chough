---
change: a-blocking-bug-stops-only-a-session-it-is-related-to
artifact: context
---

# Context

Jackdaw PR 184 could not move from test to accept. The refusal was refused:blocking-bugs for two Web-performance bugs filed by other people. Those bugs were not on the session's bugs list and shared no criterion, requirement, or case with it. One of them names no criterion, requirement, case, or evidence.

On main 8abb5bb, open_blocking returns every open bug whose scope is blocking. forward_step and assess both use that list. REQ-cli-054 said an open blocking bug stops leaving test and did not say which session.

Ruled out: keeping the repo-wide stop, stopping every session until a nameless bug is linked, matching on module or native change (a bug has neither field), and adding a fourth scope.
