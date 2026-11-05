---
change: let-a-session-started-after-legacy-history-adoption-write-events-while-the-adopted-legacy-log-stays-guarded
artifact: context
---

# Context

Adopting sessions on this repository (DRIVE-12) went in this order: `history adopt --apply`, then `session adopt`, then `session new <key>` for the next piece of work. In the new session, `stage reset` failed with "legacy history anchor changed; restore the recorded legacy bytes before writing events".

The cause: `session new` gives the new session a cursor with no legacy anchor (`legacy: null`). Every event write compares that against the anchor of the legacy log that is still present, so the comparison always fails. The adopted session itself is fine, because it holds the anchor and `stage reset` carries it into each new cycle.

So in any project that adopted a legacy log, every session started afterwards fails on its first transition. REQ-history-006's acceptance criterion, that two branches advancing different sessions merge without conflict, was tested only in projects that never had a legacy log.
