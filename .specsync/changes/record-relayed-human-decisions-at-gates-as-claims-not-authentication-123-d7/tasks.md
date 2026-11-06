---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: tasks
---

# Tasks

- [ ] Commit the failing tests (tests/relay.rs, tests/stages.rs expectations).
- [ ] Add `relayed_by` to transitions with payload validation (REQ-history-010).
- [ ] Add `chough checkpoint --close-gate` and its refusals (REQ-cli-050).
- [ ] Name `--close-gate` in the awaiting-human refusal and waiting action (REQ-cli-030).
- [ ] Narrow the wording: README, skill and installed copy, docs/cli.md, docs/FRD.md,
      docs/architecture.md, docs/walkthrough.md and cli.spec.md invariant 2; add the
      KNOWN_PREVIOUS digests and a CHANGELOG entry.
- [ ] Add and run the cases in `.chough/cases/cli.relayed-decisions.cases.toml`.
- [ ] Run verify locally and on Linux.

Human gates, as plain text: scope approval by Leif via the orc; implementation review
by Leif via the orc.
