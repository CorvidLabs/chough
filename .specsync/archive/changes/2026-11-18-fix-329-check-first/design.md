---
change: fix-329-check-first
artifact: design
---

# Design

`bug add` already holds the project lock from its start. Before `create` writes anything, it now reads the active session, as the join does, and, unless no session is active, checks the session with the new bug's link added: the same validation and rendering `save_checkpoint` runs, with nothing written. A session that cannot take it refuses the add, naming the session, the limit and how to go on, with every file unchanged. The join after `create` is unchanged; under the held lock nothing can fill the session in between.

The check uses a placeholder ID of the longest shape `create` can give the bug, so a name suffixed on collision cannot make the real save larger than the one checked.
