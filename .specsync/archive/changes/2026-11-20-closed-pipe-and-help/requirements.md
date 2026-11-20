---
change: closed-pipe-and-help
artifact: requirements
---

# Requirements

No requirement changes (`--no-spec-change`). No requirement says a command panics or exits 101 when its reader goes away, or that help depends on its place among the global options; docs/cli.md's sentence on `-V` is corrected to match the helper. The cases link REQ-cli-021 (the focused next action, the issue's `chough next | head -1`) and REQ-cli-019 (where the existing help and CLI-reference cases are linked).
