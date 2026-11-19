---
change: fix-334
artifact: context
---

# Context

GitHub #334, a follow-up from the tester's check of #331 (#271). With `CHOUGH_SESSION` exported in a project that has no session yet, every command refused with `refused:session-not-open`, naming `chough session list`, which lists nothing. It did not give the usual guidance to run `chough session new <key>`.

Leif approved the issue's expectation, relayed by hc t4c on 2026-11-18 ("Approve"): the refusal names the variable and gives the session-new guidance.
