---
change: bug-add-leaves-the-session
artifact: research
---

# Research

Options weighed (draft-r2a, read by fable-lead):
- (A, chosen) The link lives on the bug: `bug add` writes `session: <active id>` into the new bug's record and never writes the session. A session's bugs are its checkpoint's list followed by the bugs filed in it. Removes the full-session refusal, `checkpoint::check_save` and the per-bug session save; adds no refusal. A throwaway prototype merges cleanly and takes 0.03 to 0.10 s at 400 observations (0.04 s with `--observation`).
- (B) Keep the session write: render once (about 2.5 s at 400 observations) and a `session.md` merge driver that unions `bugs:`, which needs a `.gitattributes` line, `doctor --fix` in each clone and its own handling of an unmergeable front matter; a clone without it still conflicts.
- Not offered: reverting #343's auto-link (reopens #329, against Leif's decision); a link file per bug in the session's folder (a new record kind).
- Rejected inside (A): counting filed bugs only in the story and the blocking check, which would hide from `chough checkpoint` the bugs main shows today.

Prototype sweep (`cargo test --no-fail-fast`): 966 passed, 2 failed, both in `tests/bugs.rs`: the full-session refusal test, and an order flip in `a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story`, fixed by not joining a bug filed in the active session on `bug update --observation`.
