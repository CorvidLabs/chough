---
change: record-the-acting-agent-model-and-hc-terminal-on-each-transition-evidence-entry-and-bug-201
artifact: context
---

# Context

Leif's decision on issue #201 (2026-11-10, recorded on the issue by the coordinator): Chough stays current-agent-first, but it should record who did each step, now that work routinely runs across several agents through hc/Herdr and the `chough-*` templates. Each stage transition, evidence entry and bug records the acting agent, its model when known and its hc terminal; `next` and status show who made the last transition and who recorded each piece of evidence; recording is automatic where the environment exposes it, with a flag or env override, and never fails a command when the identity is unknown. Not in scope: handoff, stage ownership or dispatch.

The HI criterion is LIFE-17, captured in PR #212 (the 1.0 roadmap, owned by another worker) and not yet on main. This change's trace names it; `hi check` here does not know it until #212 merges.

Only markers observed in this session are detected automatically: `CLAUDECODE` (Claude Code) and `HC_TERMINAL_ID` (hc). Kimi's earlier probe found no Kimi-specific variable in its shell, so other hosts name themselves through `CHOUGH_ACTOR`, for example in their hc templates. Rejected: guessing host variables that were never observed.
