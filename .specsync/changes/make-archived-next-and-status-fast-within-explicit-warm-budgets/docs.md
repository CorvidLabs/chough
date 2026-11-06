---
change: make-archived-next-and-status-fast-within-explicit-warm-budgets
artifact: docs
---

# Docs

After approval, update:

- `hi/drive.md` with Leif's approved numeric warm-budget criterion, using the next free integrated ID.
- `docs/FRD.md` and the cli semantic delta with REQ-cli-053 and its DRIVE-2 trace.
- `docs/cli.md` with cached versus live read states, cache location/trust/removal, cold fallback, the exact warm measurement conditions, and the unchanged live behavior of stage/timing/retention/verification.
- `CHANGELOG.md` with the user-visible latency improvement and its archived/read-only limits.
- `.chough/cases/cli.next-status-latency.cases.toml` with focused requirement bindings after the scoped native check materializes the requirement.

The final handoff must name any remaining cold/active native latency, Fledge version mismatch, unobserved platform results and human gates. No documentation may claim that Chough authenticates an archive itself or that a warm report proves a later live gate passed.
