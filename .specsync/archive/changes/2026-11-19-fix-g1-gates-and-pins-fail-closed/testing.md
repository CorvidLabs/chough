---
change: fix-g1-gates-and-pins-fail-closed
artifact: testing
---

# Testing

One test per issue, each failing on main before the fix:

- #401 (`tests/stages.rs`, `tests/checkpoint.rs`): a session file edited by hand to
  hold `no_native_change: " "`, a `<placeholder>`, or both fields is refused by
  `next`, `checkpoint` and `stage advance` with every file byte-identical;
  `session list` reports it not readable; `doctor` runs; another session works.
- #393 (`tests/session_pin.rs`): `inbox list` under `--session` and under
  `CHOUGH_SESSION` lists the named session's entry, not the pointer's.
- #372 (`tests/session_pin.rs`, `tests/sessions.rs`): the stderr line with the
  variable set and its absence without; shelve by alias, by bare slug, by a folder
  name that is also another session's alias, and of an unreadable session.
- #369 (`tests/change.rs`): with the pointer elsewhere, `change ship --session`
  closes the gate and moves the stage in the named session; the pointer's file is
  byte-identical.
- #400 (`tests/change.rs`): 1,001 commits after the duplicate is still
  `refused:corrupt-history`; a base commit the repository lacks is
  `refused:history-unchecked`.
- REQ-cli-057 (`tests/change.rs`, after #407): a credential sample and a control
  character in each checked argument, with and without `--redact`.

Then every verify step, and the cases run on macOS.
