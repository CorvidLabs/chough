---
change: remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/82 (Chough bot review 5763662494 on #77
at 10a1439): if a session is archived while `chough stage advance` runs, the advance
writes its `EVT-` event into the session's history cycle, then its save is refused
("archived or removed while this command ran; nothing was saved"), leaving an orphan
event no checkpoint references. REQ-history-002 already makes such an event an
unselected orphan that never counts as a completed transition.

Since 10a1439, #77's writer lock means Chough's own `session use` and `session new`
wait for a running `stage advance`, so only something outside Chough (a Git checkout, a
script) can move the file mid-command. Leif chose on 2026-11-04 ("Test the lock + clean
up"): add a regression test that `session use` waits, and when a save is refused because
the session file vanished, remove the events that command published. "Check before
publishing" and "regression test only" were offered and not chosen. This branch is
stacked on #77 at 75a732d.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes, his brief to
fix issues 81 to 88 one PR each, and his #82 choice.
