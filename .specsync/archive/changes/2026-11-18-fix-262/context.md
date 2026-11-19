---
change: fix-262
artifact: context
---

# Context

GitHub #262: a FIFO named like a bug record in `.chough/bugs/` makes `chough status` and `chough next` block for ever, while `chough bug list` answers. A FIFO in place of an open session's `session.md` makes that session vanish from `chough session list`, with no reason and no recovery named. Both reproduce on main at 8abb5bb.

REQ-history-006 already says a session that cannot be read stops no other command and that `session list` reports it with the reason and the recovery `chough session shelve <id>`. The session part of this change makes the code do that for a record that is not a regular file. For the bugs folder no requirement names non-regular files. `chough bug list` already skips them, and the readers `status` and `next` use did not, so this change makes those readers skip them the same way. Whether skipping is the right answer there, rather than refusing with the path named, is put to Leif at the scope gate.

Out of scope: `chough doctor` does not report such a file. A new doctor finding is new behaviour and needs a sentence in REQ-history-012 first.
