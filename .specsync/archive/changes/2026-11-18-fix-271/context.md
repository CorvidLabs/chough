---
change: fix-271
artifact: context
---

# Context

GitHub #271, from the 0.6.0 swarm report (#273, s3). Every session-scoped write resolves the one shared pointer, `.chough/local/active-session`. So when two agents work in one worktree and one runs `chough session use` for its own session, the other agent's next `evidence add`, `checkpoint` or `bug add` lands in the wrong session, silently and with exit 0. Only `chough inbox add` took a `--session`.

Leif's decision (2026-11-18, planning interview, recorded on #271): pin per shell. A `CHOUGH_SESSION` environment variable and a `--session <id>` flag on every write command make each agent write to its own session, whatever the shared pointer says, and hc agent launches set the variable. He then approved the requirement wording in REQ-cli-037's delta ("Approve") and chose "Yes, same PR" for the hc templates: the five versioned templates in `integrations/` tell the agent to export `CHOUGH_SESSION` for its session. Both were relayed by hc t4c.
