---
change: fix-325
artifact: design
---

# Design

`evidence::record` takes the project's writer lock (`checkpoint::lock_for_update`) before anything else: the entry's name, its create-new write, its side files and the session save. A busy lock therefore refuses before any file is written. `cases run` calls `record` only after its case has run, so the case still never runs under the lock. The lock is held through the session save, as since #317.
