---
change: fix-437
artifact: requirements
---

# Requirements

Covers REQ-history-013: a direct-native-write entry's `before_state` and
`after_state` are `none` when `state.json` is absent — present as lines
in the Timeline text and as `none` in the entry's sentence, never
omitted or "unknown". Observable outcome: a direct write recorded while
state.json is absent shows `before_state: none` and `after_state: none`
and its text says the state stays none.