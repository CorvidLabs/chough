---
change: fix-438
artifact: context
---

# Context

Found by the 0.6.2 documentation audit and filed as chough#438.
REQ-cli-061 says `chough gate record` "SHALL close all the named gates or
none". Once its pre-checks pass, it closes them one at a time, each as
its own `chough checkpoint --close-gate` subprocess (src/gates.rs about
lines 1084-1127). If a later close is refused — the reachable case is a
later save hitting the session record or Timeline bound — the earlier
gates stay closed and the error asks to record the remaining gates
again. The pre-checks already cover relay completeness, digest changes,
verifying changes and the native-step count; the bound hit is what
slips through.