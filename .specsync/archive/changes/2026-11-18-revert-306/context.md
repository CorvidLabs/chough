---
change: revert-306
artifact: context
---

# Context

#306 (the fix for #260, merged 2026-11-18 as be6c9f1) changed how a new record is
written: the bytes go to a temporary file under `.chough/local/`, which is then
hard-linked to the record's name. Two regressions were found before 0.6.1:

- #313: a file system without hard links, such as exFAT, refuses the link
  ("Operation not supported (os error 45)"), so `chough init`, `bug add`,
  `journal add` and `session new` fail, and `init` leaves a half-made `.chough/`.
- The temporary file is written before the writer lock is taken, and a writer
  holding the lock sweeps it as left by a stopped write. The tester's parallel
  checkpoint and evidence storms failed in 3 of 4 runs on main and 0 of 3 on
  0.6.0.

Leif's decision, relayed by his planning session (hc t4c) on 2026-11-18:
"Revert #306". This change reverts #306's code, its test, its case and its
changelog fragment. It keeps #305's non-blocking reader and #308's journal
lock. The archived change fix-260 and its session story stay as history. #260
reopens for 0.6.2, to be redone with an exFAT test and a parallel sweep test
from the start.

A fix-forward fallback for #313 was started and is parked, unpushed.
