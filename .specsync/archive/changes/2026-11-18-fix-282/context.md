---
change: fix-282
artifact: context
---

# Context

GitHub #282. A session's front matter has a `pr` field, which the leak guard checks on every save, but no command set it. `checkpoint --help` had no `--pr`, so linking a session to its pull request meant hand-editing `session.md`.

Leif's decision (2026-11-18, planning interview, recorded on #282 and relayed by hc t4c as the scope approval) was to add `chough checkpoint --pr <number or URL>`, with the existing `pr` validation and a Timeline entry, like every other checkpoint field. No field setter writes a Timeline entry today, and the entry kinds are a closed list in REQ-history-002. Asked about that, he answered "No Timeline entry": `--pr` sets `pr` with the existing validation, like `--objective` and `--title`, a cli-only change (relayed by hc t4c, 2026-11-18).
