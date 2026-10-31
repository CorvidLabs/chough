# Decisions

These entries distinguish human choices from implementation choices. They are
context for later work, not authenticated approval records.

| ID | Decision | Basis and consequence |
|---|---|---|
| D-001 | Name the project Chough, pronounced chuff. | Explicit human choice; private `CorvidLabs/chough`. |
| D-002 | Make v1 current-agent-first. | Explicit human choice; skills guide the active agent. Rune coordination is deferred. |
| D-003 | Keep most behavior in skills and Markdown; use a small Rust helper. | Accepted human direction; no model client, hosted service, or custom runtime. |
| D-004 | Preserve native ownership. | Accepted architecture: HI owns intent, SpecSync owns changes/contracts/lessons, Fledge owns task execution. |
| D-005 | Bootstrap with native structural checks; do not invent approval (superseded by D-008 for new work). | Implementation choice within authorized first development. Native SDD is initialized with its default disabled policy; no approved/archived lifecycle is claimed. |
| D-006 | Store a focused session and revision/environment-bound observations. | First-slice implementation choice; durable criteria remain unchanged as achievement changes. |
| D-007 | Expose adapters through bounded inputs and native tool output. | Implementation choice; adding an adapter requires compatibility tests and a clear ownership boundary. |
| D-008 | Adopt the full SpecSync workflow-v2 lifecycle; meaningful files need a native change. | Human direction (2026-10-31 brief) and approval of `evolve-chough-into-a-resumable-stage-workflow`. Supersedes D-005 for new work. |
| D-009 | Lifecycle writers and CI use SpecSync 6.1.1. | Implementation choice before the change: PATH already resolved 6.1.1; the v6.1.1 Action tag has assets and needs an explicit version. Fledge pins unchanged. |
| D-010 | Qualify APS 1.1.0 and defer it; `.chough/session.json` stays the only checkpoint. | Approved with the change. Observed lost concurrent updates without error, unknown fields accepted, an implicit global root and no Windows binary. Reconsider when those change. |
| D-011 | Test definitions use TOML profile `chough-testing/1`, not 3md. | Approved with the change. 3md attributes are single-line strings, its tools ignore unknown fields, and SpecSync reads module `.3md` as lessons. |
| D-012 | `cases run` and `validate` execute bound commands only on explicit invocation, without a shell. | Approved with the change; replaces the bootstrap invariant that the helper never runs tests. Vacuous or environment failures are not-run. |
| D-013 | Monitor findings are captured locally by default; propose-change only prints a command. | Approved default; whether monitors may start investigations stays unsettled. |
| D-014 | One canonical stage list in the helper, documented in `references/stages.md`. | Implementation choice; a test keeps them in step. |
| D-015 | Crate-internal modules use `pub(super)`. | Implementation choice: the binary has no library target, and the contract describes commands rather than Rust symbols. |
| D-016 | `docs/NEXT_CHANGE.md` is superseded, not implemented. | Human redirected the next work; observation reuse after metadata commits stays unapproved. |

Changing product intent requires updating HI through its supported flow, preserving
permanent IDs. Changing an implementation choice requires a recorded reason and
updated requirements or contracts where behavior changes.
