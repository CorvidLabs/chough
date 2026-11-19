---
change: fix-271
artifact: requirements
---

# Requirements

REQ-cli-037 (Active session checkpoint) gains the per-shell pin: a command uses the session its `--session <id>` option or the `CHOUGH_SESSION` variable names, resolved as `chough session use` resolves an ID, alias or bare slug. A named session that is not open refuses with nothing written, and the pin never changes the pointer. `init --session` and `inbox add --session` keep their meanings, and `session new` and `session use` ignore the variable. Two acceptance criteria are added. The wording is the one Leif approved (relayed by hc t4c, 2026-11-18). This is a public contract change: a new global option and a new environment variable.
