---
change: bug-add-leaves-the-session
artifact: design
---

# Design

- `src/bugs.rs` (cli): a bug's `session` key, written by `bug add` when a session is active, after `found_by`, and only when set. `of_session(root, session)` gives the session's bugs: the checkpoint's list, then the bugs whose `session` is the session's ID, ordered by `created` and then by name, each once; a bug record that cannot be read is left out there, as doctor names it. `related_to` also counts a bug whose `session` names the session. `add` drops `session_takes_a_bug` and the join; the update join is skipped for a bug filed in the active session.
- `src/checkpoint.rs` (cli): `check_save` removed, which gives `save_checkpoint` back the doc comment #343 split from it; the `checkpoint` output's `bugs` is the session's bugs.
- `src/main.rs` (cli): `context` lists the session's bugs, in JSON and Markdown.
- `src/sessions.rs` (history): the story's `## Bugs` names the session's bugs.
- A bug whose `session` names an archived session, or one that no longer exists, matches no open session: nothing lists it and nothing refuses or reports it. No new refusal or finding.
