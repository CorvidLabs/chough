---
change: fix-317
artifact: design
---

# Design

The three commands share one save, the function in `src/evidence.rs` that writes the entry and then adds its observation reference to the session. That function takes the project's writer lock (`checkpoint::lock_checkpoint`) before it reads `session.md`, and keeps it through `save_checkpoint`, as it already does for the rest of the process. `save_checkpoint` takes the same lock re-entrantly, so nothing else changes.

`bug add` and `bug update` given `--observation` with an entry of the active session add the bug to that session in `bugs::join_active_session`. Both commands already take the writer lock at their start, so that read is under the lock, and `src/bugs.rs` does not change; a test guards it.

`cases run` runs its case before the evidence function, so it still holds no lock while the case runs, and a long case never makes another writer wait past the 30-second bound of REQ-cli-040. The observation-cap check that runs before the case (REQ-cli-032) stays an early refusal without the lock; `save_checkpoint` still validates the bound under the lock.
