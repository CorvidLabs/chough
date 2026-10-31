---
change: parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first
artifact: docs
---

# Docs

`docs/cli.md` gains one sentence under records and limits: native tool output is
parsed in full up to 4 MiB and displayed up to 16 KiB; `native_summary` carries the
parsed state and next action. No other documentation changes.
