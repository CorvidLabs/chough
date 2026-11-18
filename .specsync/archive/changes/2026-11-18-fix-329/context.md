---
change: fix-329
artifact: context
---

# Context

GitHub #329, a follow-up to #304 from the tester's check of #304 on 2026-11-18. Since #304, a blocking bug stops a session only when the two are related. A blocking bug filed with no observation, criterion, requirement or case is related to no session, so it no longer stops even the session that filed it.

Leif's decision (2026-11-18, planning interview, recorded on #329 and relayed by hc t4c as the scope approval) is auto-link. `chough bug add` while a session is active relates the new bug to that session, so a blocking bug stops it. A bug filed with no session active stays unlinked and gets the existing warning, `chough next`'s "Other blocking bugs: … (do not block this session)".

The link is the session's bugs list, `checkpoint --bug`, which REQ-cli-054 already counts as related. `chough bug add` already joined the active session when its `--observation` named that session's evidence entry (REQ-cli-012); it now joins whenever a session is active. `chough bug update` is unchanged.

Recreated on Leif's word, "Recreate on my word" (relayed by hc t4c on 2026-11-18), to declare `tests/records.rs`. Its alias test reads the session's `bugs:` list, which now also holds the bug that test files, so it reads the list from the checkpoint. The first draft was never pushed. The verify lane's audit refused the undeclared path although `change check` passed.
