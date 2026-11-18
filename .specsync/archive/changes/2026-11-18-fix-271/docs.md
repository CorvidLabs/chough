---
change: fix-271
artifact: docs
---

# Docs

`docs/cli.md` describes the global `--session <id>` option and `CHOUGH_SESSION`: which session a command uses, the refusal for a session that is not open, that the pointer never moves, and the commands that keep their own `--session` or ignore the variable. Each hc template in `integrations/` tells its agent to export `CHOUGH_SESSION` for its session.
