---
change: reuse-specsync-s-authentication-of-an-unchanged-committed-archive-so-next-status-stage-moves-and-timing-skip-the-slow
artifact: context
---

# Context

Leif's brief t811u (5:03 PM MST, confirmed as his criteria at 5:24 PM MST) reports that on Raven every `chough next`, `status`, stage move and `timing` waits on one `specsync change status <id> --json`, which takes about 47 seconds for an archived change, and that three `stage advance` calls in a row each re-read native status. Timing on the Raven proof checkout with SpecSync 6.1.2 measured next 48.44s, status 46.4s, stage advance 47.63s (gate native:archived) and timing 120.01s with its budget exhausted.

The slow part is SpecSync authenticating the archive (`validate_archived_integrity_inner` in SpecSync 6.1.2). The archive's committed bytes rarely change after merge, so the authentication can be reused while they stay identical.

Constraints: native SpecSync state still decides stage gates; Chough still refuses to record human approval or review; REQ-cli-024 forbids persisting reuse and REQ-cli-028 forbids a persistent cache, so both are amended in this change rather than contradicted.
