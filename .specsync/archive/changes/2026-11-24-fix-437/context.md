---
change: fix-437
artifact: context
---

# Context

Found by the 0.6.2 documentation audit and filed as chough#437.
REQ-history-013 says a direct lifecycle write's `before_state` and
`after_state` are `none` when `state.json` is absent. The Timeline
writer leaves those lines out entirely when the state is absent
(src/session_md.rs, the `part` closure skips `None`, lines about 730),
and the entry's text says "unknown" (src/direct_writes.rs, the `state`
closure about line 116). docs/cli.md documents `none`.