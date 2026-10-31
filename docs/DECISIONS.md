# Decisions

These entries distinguish human choices from implementation choices. They are
context for later work, not authenticated approval records.

| ID | Decision | Basis and consequence |
|---|---|---|
| D-001 | Name the project Chough, pronounced chuff. | Explicit human choice; private `CorvidLabs/chough`. |
| D-002 | Make v1 current-agent-first. | Explicit human choice; skills guide the active agent. Rune coordination is deferred. |
| D-003 | Keep most behavior in skills and Markdown; use a small Rust helper. | Accepted human direction; no model client, hosted service, or custom runtime. |
| D-004 | Preserve native ownership. | Accepted architecture: HI owns intent, SpecSync owns changes/contracts/lessons, Fledge owns task execution. |
| D-005 | Bootstrap with native structural checks; do not invent approval. | Implementation choice within authorized first development. Native SDD is initialized with its default disabled policy; no approved/archived lifecycle is claimed. |
| D-006 | Store a focused session and revision/environment-bound observations. | First-slice implementation choice; durable criteria remain unchanged as achievement changes. |
| D-007 | Expose adapters through bounded inputs and native tool output. | Implementation choice; adding an adapter requires compatibility tests and a clear ownership boundary. |

Changing product intent requires updating HI through its supported flow, preserving
permanent IDs. Changing an implementation choice requires a recorded reason and
updated requirements or contracts where behavior changes.
