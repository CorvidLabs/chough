---
change: fix-325
artifact: context
---

# Context

GitHub #325, found by the macOS swarm test (s3-F4) and seen again in the tester's check of #324. When `chough evidence add` waits 30 seconds for the writer lock and then refuses as `refused:checkpoint-busy` ("nothing was saved"), its evidence entry file has already been written, so a rerun creates a duplicate entry. REQ-cli-040 already says a busy writer refuses with nothing saved. The evidence save (`evidence::record`, shared by `evidence add`, `cases run` and `cases record`) writes its entry by create-new before it takes the lock to add the observation reference (#317).

Leif's decision (2026-11-18, planning interview, recorded on #325 and relayed by hc t4c as the scope approval): approved as a fix. A `refused:checkpoint-busy` evidence add leaves nothing behind, so a rerun doesn't duplicate the entry.
